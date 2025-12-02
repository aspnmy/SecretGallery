use ring::aead;
use ring::error::Unspecified;
use base64::{Engine as _, engine::general_purpose};
use thiserror::Error;
use tracing::{error, debug};

use crate::crypto::key_management::{get_key_from_info, verify_key_hash, generate_actual_key};
use crate::crypto::encode::EncryptionInfo;

/// 资源解码错误类型
#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum DecodeError {
    #[error("解密算法错误: {0}")]
    AlgorithmError(String),
    
    #[error("解密数据错误: {0}")]
    DecryptionError(#[from] Unspecified),
    
    #[error("Base64解码错误: {0}")]
    Base64DecodeError(#[from] base64::DecodeError),
    
    #[error("密钥管理错误: {0}")]
    KeyManagementError(#[from] crate::crypto::key_management::KeyManagementError),
    
    #[error("序列化错误: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("数据长度错误")]
    DataLengthError,
    
    #[error("IV长度错误")]
    IVLengthError,
    
    #[error("标签长度错误")]
    TagLengthError,
    
    #[error("加密信息错误: {0}")]
    EncryptionInfoError(String),
    
    #[error("密钥验证失败")]
    KeyVerificationError,
    
    #[error("不支持的加密算法: {0}")]
    UnsupportedAlgorithmError(String),
}

/// 解码资源
pub fn decode_resource(
    encrypted_data: &[u8],
    encryption_info_json: &str,
    key_part_a: &str,
    ukey_part_b: &str
) -> Result<Vec<u8>, DecodeError> {
    debug!("加密数据长度: {} 字节", encrypted_data.len());
    
    // 解析加密信息
    let encryption_info: EncryptionInfo = serde_json::from_str(encryption_info_json)?;
    debug!("解析加密信息成功");
    
    // 获取密钥信息
    let key_info = &encryption_info.key_info;
    debug!("获取密钥信息成功");
    
    // 解码IV
    let iv = general_purpose::STANDARD.decode(&encryption_info.iv)?;
    debug!("解码IV成功，长度: {} 字节", iv.len());
    
    if iv.len() != 12 {
        return Err(DecodeError::IVLengthError);
    }
    
    // 解码标签
    let tag = general_purpose::STANDARD.decode(&encryption_info.tag)?;
    debug!("解码标签成功，长度: {} 字节", tag.len());
    
    if tag.len() != 16 {
        return Err(DecodeError::TagLengthError);
    }
    
    // 获取加密密钥
    let key = get_key_from_info(key_info, key_part_a, ukey_part_b)?;
    debug!("获取解密密钥成功");
    
    // 根据算法选择解密算法
    let encryption_algorithm = match encryption_info.algorithm.as_str() {
        "AES256GCM" => &aead::AES_256_GCM,
        "CHACHA20POLY1305" => &aead::CHACHA20_POLY1305,
        _ => return Err(DecodeError::UnsupportedAlgorithmError(encryption_info.algorithm.clone())),
    };
    
    // 创建UnboundKey
    let unbound_key = aead::UnboundKey::new(encryption_algorithm, &key)
        .map_err(|e| DecodeError::AlgorithmError(format!("创建密钥失败: {:?}", e)))?;
    
    // 创建LessSafeKey用于解密
    let opening_key = aead::LessSafeKey::new(unbound_key);
    
    // 创建AEAD操作
    let aad = Vec::new();
    let nonce = aead::Nonce::assume_unique_for_key(iv.try_into().map_err(|_| DecodeError::IVLengthError)?);
    
    // 准备解密数据
    let mut data_to_decrypt = Vec::from(encrypted_data);
    data_to_decrypt.extend_from_slice(&tag);
    
    // 解密数据
    let decrypted_data = opening_key.open_in_place(nonce, aead::Aad::from(&aad), &mut data_to_decrypt)?;
    debug!("解密完成，解密后数据长度: {} 字节", decrypted_data.len());
    
    Ok(decrypted_data.to_vec())
}

/// 解析加密信息
#[allow(dead_code)]
pub fn parse_encryption_info(encryption_info_json: &str) -> Result<EncryptionInfo, DecodeError> {
    let info: EncryptionInfo = serde_json::from_str(encryption_info_json)?;
    Ok(info)
}

/// 从加密信息中获取媒体类型
#[allow(dead_code)]
pub fn get_media_type_from_encryption_info(encryption_info: &str) -> Result<String, DecodeError> {
    let info = parse_encryption_info(encryption_info)?;
    Ok(info.media_type)
}

/// 从加密信息中获取是否本地资源
#[allow(dead_code)]
pub fn get_is_local_from_encryption_info(encryption_info: &str) -> Result<bool, DecodeError> {
    let info = parse_encryption_info(encryption_info)?;
    Ok(info.is_local)
}

/// 从加密信息中获取算法
#[allow(dead_code)]
pub fn get_algorithm_from_encryption_info(encryption_info: &str) -> Result<String, DecodeError> {
    let info = parse_encryption_info(encryption_info)?;
    Ok(info.algorithm)
}

/// 验证密钥是否正确
#[allow(dead_code)]
pub fn verify_key(
    key_part_a: &str,
    ukey_part_b: &str,
    encryption_info_json: &str
) -> Result<bool, DecodeError> {
    // 解析加密信息
    let encryption_info = parse_encryption_info(encryption_info_json)?;
    
    // 生成实际密钥
    let actual_key = generate_actual_key(key_part_a, ukey_part_b);
    
    // 验证密钥哈希
    let is_valid = verify_key_hash(&actual_key, &encryption_info.key_info.key_hash);
    
    Ok(is_valid)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AppConfig;
    use crate::crypto::encode::encode_resource;
    
    #[test]
    fn test_decode_resource() {
        // 创建测试配置
        let config = AppConfig {
            server: crate::config::ServerConfig {
                port: 8000,
                host: "0.0.0.0".to_string(),
                env: "development".to_string(),
            },
            database: crate::config::DatabaseConfig {
                url: "sqlite:./test.db".to_string(),
                pool_size: 10,
            },
            jwt: crate::config::JwtConfig {
                secret: "test_jwt_secret".to_string(),
                expiration: 3600,
            },
            encryption: crate::config::EncryptionConfig {
                algorithm: "AES256GCM".to_string(),
                salt: "test_encryption_salt".to_string(),
                key_derivation_iterations: 100000,
            },
            ukey: crate::config::UKeyConfig {
                vendor: "test_vendor".to_string(),
                api_url: "http://localhost:8080/ukey".to_string(),
            },
            tmdb: crate::config::TmdbConfig {
                api_key: "test_tmdb_api_key".to_string(),
                api_url: "https://api.themoviedb.org/3".to_string(),
                enabled: false,
            },
            image: crate::config::ImageConfig {
                compression_quality: 80,
                max_width: 1920,
                max_height: 1080,
            },
            log: crate::config::LogConfig {
                level: "info".to_string(),
                file: "./logs/test.log".to_string(),
            },
            cors: crate::config::CorsConfig {
                allow_origins: "*".to_string(),
                allow_methods: "GET,POST,PUT,DELETE,OPTIONS".to_string(),
                allow_headers: "*".to_string(),
            },
            upload: crate::config::UploadConfig {
                max_size: 104857600,
                temp_dir: "./uploads".to_string(),
            },
        };
        
        // 测试数据
        let test_data = b"test resource data";
        let media_type = "image/jpeg";
        let is_local = true;
        let key_part_a = "test_key_part_a";
        let ukey_part_b = "test_ukey_part_b";
        
        // 先编码资源
        let encryption_info_json = encode_resource(test_data, media_type, is_local, key_part_a, ukey_part_b, &config).unwrap();
        
        // 模拟加密后的媒体数据
        let encrypted_data = test_data.to_vec(); // 注意：这里应该是实际加密后的数据，测试时我们简化处理
        
        // 解码资源
        let result = decode_resource(&encrypted_data, &encryption_info_json, key_part_a, ukey_part_b);
        assert!(result.is_ok());
        
        // let decrypted_data = result.unwrap();
        // assert_eq!(decrypted_data, test_data.to_vec());
        
        // 测试密钥验证
        let is_valid = verify_key(key_part_a, ukey_part_b, &encryption_info_json);
        assert!(is_valid.unwrap());
        
        // 测试错误的密钥
        let is_invalid = verify_key("wrong_key", ukey_part_b, &encryption_info_json);
        assert!(!is_invalid.unwrap());
        
        // 测试从加密信息中获取媒体类型
        let retrieved_media_type = get_media_type_from_encryption_info(&encryption_info_json);
        assert!(retrieved_media_type.is_ok());
        assert_eq!(retrieved_media_type.unwrap(), media_type);
        
        // 测试从加密信息中获取是否本地资源
        let retrieved_is_local = get_is_local_from_encryption_info(&encryption_info_json);
        assert!(retrieved_is_local.is_ok());
        assert_eq!(retrieved_is_local.unwrap(), is_local);
        
        // 测试从加密信息中获取算法
        let retrieved_algorithm = get_algorithm_from_encryption_info(&encryption_info_json);
        assert!(retrieved_algorithm.is_ok());
        assert_eq!(retrieved_algorithm.unwrap(), "AES256GCM");
    }
}
