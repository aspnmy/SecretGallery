use ring::aead;
use ring::error::Unspecified;
use base64::{Engine as _, engine::general_purpose};
use serde::{Serialize, Deserialize};
use thiserror::Error;
use tracing::{error, debug};

use crate::config::{AppConfig};
use crate::crypto::key_management::{self, KeyInfo, get_key_from_info, generate_key_info, get_encryption_salt, get_key_derivation_iterations, get_encryption_algorithm};

/// 资源编码错误类型
#[derive(Error, Debug)]
pub enum EncodeError {
    #[error("加密算法错误: {0}")]
    AlgorithmError(String),
    
    #[error("加密数据错误: {0}")]
    EncryptionError(#[from] Unspecified),
    
    #[error("Base64解码错误: {0}")]
    Base64DecodeError(#[from] base64::DecodeError),
    
    #[error("密钥管理错误: {0}")]
    KeyManagementError(#[from] key_management::KeyManagementError),
    
    #[error("序列化错误: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("数据长度错误")]
    DataLengthError,
    
    #[error("IV长度错误")]
    IVLengthError,
    
    #[error("标签长度错误")]
    TagLengthError,
}

/// 加密信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EncryptionInfo {
    pub key_info: KeyInfo,
    pub iv: String,
    pub tag: String,
    pub algorithm: String,
    pub media_type: String,
    pub is_local: bool,
}

/// 编码资源
pub fn encode_resource(
    data: &[u8],
    media_type: &str,
    is_local: bool,
    key_part_a: &str,
    ukey_part_b: &str,
    config: &AppConfig
) -> Result<String, EncodeError> {
    debug!("数据长度: {} 字节", data.len());
    
    // 获取加密算法
    let algorithm = get_encryption_algorithm(config);
    debug!("使用加密算法: {}", algorithm);
    
    // 生成IV
    let iv = key_management::generate_iv();
    debug!("生成IV: {:?}", iv);
    
    // 保存IV的副本，因为后面需要使用
    let iv_copy = iv.clone();
    
    // 获取加密盐值
    let salt = get_encryption_salt(config);
    debug!("使用盐值: {:?}", salt);
    
    // 获取密钥派生迭代次数
    let iterations = get_key_derivation_iterations(config);
    debug!("使用迭代次数: {}", iterations);
    
    // 生成密钥信息
    let key_info = generate_key_info(algorithm, &salt, iterations, key_part_a, ukey_part_b)?;
    debug!("生成密钥信息成功");
    
    // 获取加密密钥
    let key = get_key_from_info(&key_info, key_part_a, ukey_part_b)?;
    debug!("获取加密密钥成功");
    
    // 根据算法选择加密算法
    let encryption_algorithm = match algorithm {
        "AES256GCM" => &aead::AES_256_GCM,
        "CHACHA20POLY1305" => &aead::CHACHA20_POLY1305,
        _ => return Err(EncodeError::AlgorithmError(format!("不支持的加密算法: {}", algorithm))),
    };
    
    // 创建UnboundKey
    let unbound_key = aead::UnboundKey::new(encryption_algorithm, &key)
        .map_err(|e| EncodeError::AlgorithmError(format!("创建密钥失败: {:?}", e)))?;
    
    // 创建LessSafeKey用于加密
    let sealing_key = aead::LessSafeKey::new(unbound_key);
    
    // 创建AEAD操作
    let aad = Vec::new();
    let nonce = aead::Nonce::assume_unique_for_key(iv.try_into().map_err(|_| EncodeError::IVLengthError)?);
    
    // 加密数据
    let mut data_to_encrypt = Vec::from(data);
    let tag = sealing_key.seal_in_place_separate_tag(nonce, aead::Aad::from(&aad), &mut data_to_encrypt)?;
    debug!("加密完成，加密后数据长度: {} 字节，标签长度: {} 字节", data_to_encrypt.len(), tag.as_ref().len());
    
    // 构建加密信息
    let encryption_info = EncryptionInfo {
        key_info,
        iv: general_purpose::STANDARD.encode(&iv_copy),
        tag: general_purpose::STANDARD.encode(&tag),
        algorithm: algorithm.to_string(),
        media_type: media_type.to_string(),
        is_local,
    };
    
    // 序列化加密信息
    let encryption_info_json = serde_json::to_string(&encryption_info)?;
    debug!("加密信息序列化成功");
    
    Ok(encryption_info_json)
}

/// 从加密信息中获取媒体类型
pub fn get_media_type_from_encryption_info(encryption_info: &str) -> Result<String, EncodeError> {
    let info: EncryptionInfo = serde_json::from_str(encryption_info)?;
    Ok(info.media_type)
}

/// 从加密信息中获取是否本地资源
pub fn get_is_local_from_encryption_info(encryption_info: &str) -> Result<bool, EncodeError> {
    let info: EncryptionInfo = serde_json::from_str(encryption_info)?;
    Ok(info.is_local)
}

/// 从加密信息中获取算法
pub fn get_algorithm_from_encryption_info(encryption_info: &str) -> Result<String, EncodeError> {
    let info: EncryptionInfo = serde_json::from_str(encryption_info)?;
    Ok(info.algorithm)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AppConfig;
    
    #[test]
    fn test_encode_resource() {
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
        
        // 编码资源
        let result = encode_resource(test_data, media_type, is_local, key_part_a, ukey_part_b, &config);
        assert!(result.is_ok());
        
        let encryption_info_json = result.unwrap();
        assert!(!encryption_info_json.is_empty());
        
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
