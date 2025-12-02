use ring::{digest, pbkdf2, rand, rand::SecureRandom};
use std::num::NonZeroU32;
use base64::{Engine as _, engine::general_purpose};
use thiserror::Error;
use tracing::{error};

use crate::config::{AppConfig};

/// 密钥管理错误类型
#[derive(Error, Debug)]
pub enum KeyManagementError {
    #[error("密钥派生错误: {0}")]
    KeyDerivationError(String),
    
    #[error("密钥长度错误")]
    KeyLengthError,
    
    #[error("哈希计算错误")]
    HashError,
    
    #[error("Base64解码错误: {0}")]
    Base64DecodeError(#[from] base64::DecodeError),
    
    #[error("盐值长度错误")]
    SaltLengthError,
    
    #[error("迭代次数错误")]
    IterationCountError,
    
    #[error("随机数生成错误")]
    RandomGenerationError,
}

/// 密钥信息
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct KeyInfo {
    pub algorithm: String,
    pub salt: String,
    pub iteration_count: u32,
    pub key_hash: String,
    pub ukey_info: String,
}

/// 生成盐值
pub fn generate_salt() -> Vec<u8> {
    let mut salt = vec![0u8; 16];
    let rng = rand::SystemRandom::new();
    rng.fill(&mut salt).map_err(|_| KeyManagementError::RandomGenerationError).unwrap();
    salt
}

/// 从密钥部分A和B生成实际密钥
pub fn generate_actual_key(key_part_a: &str, key_part_b: &str) -> String {
    format!("{}{}", key_part_a, key_part_b)
}

/// 生成密钥哈希
pub fn generate_key_hash(key: &str) -> String {
    let hash = digest::digest(&digest::SHA256, key.as_bytes());
    general_purpose::STANDARD.encode(hash.as_ref())
}

/// 验证密钥哈希
pub fn verify_key_hash(key: &str, expected_hash: &str) -> bool {
    let actual_hash = generate_key_hash(key);
    actual_hash == expected_hash
}

/// 从密码生成加密密钥
pub fn derive_key_from_password(
    password: &str, 
    salt: &[u8], 
    iteration_count: u32
) -> Result<[u8; 32], KeyManagementError> {
    if salt.len() < 8 {
        return Err(KeyManagementError::SaltLengthError);
    }
    
    if iteration_count < 10000 {
        return Err(KeyManagementError::IterationCountError);
    }
    
    let iterations = NonZeroU32::new(iteration_count).ok_or(KeyManagementError::IterationCountError)?;
    
    let mut key = [0u8; 32];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        iterations,
        salt,
        password.as_bytes(),
        &mut key,
    );
    
    Ok(key)
}

/// 生成密钥信息
pub fn generate_key_info(
    algorithm: &str, 
    salt: &[u8], 
    iteration_count: u32,
    key_part_a: &str,
    ukey_part_b: &str
) -> Result<KeyInfo, KeyManagementError> {
    let actual_key = generate_actual_key(key_part_a, ukey_part_b);
    let key_hash = generate_key_hash(&actual_key);
    
    Ok(KeyInfo {
        algorithm: algorithm.to_string(),
        salt: general_purpose::STANDARD.encode(salt),
        iteration_count,
        key_hash,
        ukey_info: general_purpose::STANDARD.encode(ukey_part_b.as_bytes()),
    })
}

/// 从密钥信息中获取密钥
pub fn get_key_from_info(
    key_info: &KeyInfo,
    key_part_a: &str,
    ukey_part_b: &str
) -> Result<[u8; 32], KeyManagementError> {
    let actual_key = generate_actual_key(key_part_a, ukey_part_b);
    
    // 验证密钥哈希
    if !verify_key_hash(&actual_key, &key_info.key_hash) {
        return Err(KeyManagementError::KeyDerivationError("密钥验证失败".to_string()));
    }
    
    // 解码盐值
    let salt = general_purpose::STANDARD.decode(&key_info.salt)?;
    
    // 派生密钥
    derive_key_from_password(&actual_key, &salt, key_info.iteration_count)
}

/// 从配置中获取加密算法
pub fn get_encryption_algorithm(config: &AppConfig) -> &str {
    &config.encryption.algorithm
}

/// 从配置中获取加密盐值
pub fn get_encryption_salt(config: &AppConfig) -> Vec<u8> {
    config.encryption.salt.as_bytes().to_vec()
}

/// 从配置中获取密钥派生迭代次数
pub fn get_key_derivation_iterations(config: &AppConfig) -> u32 {
    config.encryption.key_derivation_iterations
}

/// 生成随机IV
pub fn generate_iv() -> Vec<u8> {
    let mut iv = vec![0u8; 12]; // GCM建议使用12字节IV
    let rng = rand::SystemRandom::new();
    rng.fill(&mut iv).map_err(|_| KeyManagementError::RandomGenerationError).unwrap();
    iv
}

/// 生成随机密钥
pub fn generate_random_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    let rng = rand::SystemRandom::new();
    rng.fill(&mut key).map_err(|_| KeyManagementError::RandomGenerationError).unwrap();
    key
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generate_actual_key() {
        let key_part_a = "password123";
        let key_part_b = "ukey123";
        let expected = "password123ukey123";
        let actual = generate_actual_key(key_part_a, key_part_b);
        assert_eq!(actual, expected);
    }
    
    #[test]
    fn test_generate_key_hash() {
        let key = "test_key";
        let hash1 = generate_key_hash(key);
        let hash2 = generate_key_hash(key);
        assert_eq!(hash1, hash2);
        
        let different_key = "different_key";
        let hash3 = generate_key_hash(different_key);
        assert_ne!(hash1, hash3);
    }
    
    #[test]
    fn test_verify_key_hash() {
        let key = "test_key";
        let hash = generate_key_hash(key);
        assert!(verify_key_hash(key, &hash));
        assert!(!verify_key_hash("wrong_key", &hash));
    }
    
    #[test]
    fn test_derive_key_from_password() {
        let password = "test_password";
        let salt = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let iteration_count = 100000;
        
        let key = derive_key_from_password(password, &salt, iteration_count).unwrap();
        assert_eq!(key.len(), 32);
        
        // 相同参数应生成相同密钥
        let key2 = derive_key_from_password(password, &salt, iteration_count).unwrap();
        assert_eq!(key, key2);
        
        // 不同密码应生成不同密钥
        let key3 = derive_key_from_password("different_password", &salt, iteration_count).unwrap();
        assert_ne!(key, key3);
    }
    
    #[test]
    fn test_generate_iv() {
        let iv = generate_iv();
        assert_eq!(iv.len(), 12);
        
        let iv2 = generate_iv();
        assert_ne!(iv, iv2);
    }
    
    #[test]
    fn test_generate_random_key() {
        let key = generate_random_key();
        assert_eq!(key.len(), 32);
        
        let key2 = generate_random_key();
        assert_ne!(key, key2);
    }
}
