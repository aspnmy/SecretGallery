// 密钥管理模块
pub mod key_management;

// 资源编码模块
pub mod encode;

// 资源解码模块
pub mod decode;

// 重新导出公共API
pub use key_management::{generate_key_hash, verify_key_hash, derive_key_from_password, get_key_from_info, generate_salt, generate_iv, generate_random_key};
pub use encode::{encode_resource, EncryptionInfo};
pub use decode::{decode_resource, parse_encryption_info, verify_key};

// 重新导出错误类型
pub use key_management::KeyManagementError;
pub use encode::EncodeError;
pub use decode::DecodeError;

// 统一的加密服务错误类型
pub enum CryptoError {
    /// 密钥管理错误
    KeyManagement(KeyManagementError),
    
    /// 编码错误
    Encode(EncodeError),
    
    /// 解码错误
    Decode(DecodeError),
    
    /// 其他错误
    Other(anyhow::Error),
}

// 实现错误转换
impl From<KeyManagementError> for CryptoError {
    fn from(err: KeyManagementError) -> Self {
        CryptoError::KeyManagement(err)
    }
}

impl From<EncodeError> for CryptoError {
    fn from(err: EncodeError) -> Self {
        CryptoError::Encode(err)
    }
}

impl From<DecodeError> for CryptoError {
    fn from(err: DecodeError) -> Self {
        CryptoError::Decode(err)
    }
}

impl From<anyhow::Error> for CryptoError {
    fn from(err: anyhow::Error) -> Self {
        CryptoError::Other(err)
    }
}

impl std::fmt::Display for CryptoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CryptoError::KeyManagement(err) => write!(f, "密钥管理错误: {}", err),
            CryptoError::Encode(err) => write!(f, "编码错误: {}", err),
            CryptoError::Decode(err) => write!(f, "解码错误: {}", err),
            CryptoError::Other(err) => write!(f, "其他错误: {}", err),
        }
    }
}

impl std::fmt::Debug for CryptoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CryptoError::KeyManagement(err) => write!(f, "CryptoError::KeyManagement({:?})
    详细信息: {}", err, err),
            CryptoError::Encode(err) => write!(f, "CryptoError::Encode({:?})
    详细信息: {}", err, err),
            CryptoError::Decode(err) => write!(f, "CryptoError::Decode({:?})
    详细信息: {}", err, err),
            CryptoError::Other(err) => write!(f, "CryptoError::Other({:?})
    详细信息: {}", err, err),
        }
    }
}

impl std::error::Error for CryptoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CryptoError::KeyManagement(err) => Some(err),
            CryptoError::Encode(err) => Some(err),
            CryptoError::Decode(err) => Some(err),
            CryptoError::Other(err) => Some(err.as_ref()),
        }
    }
}

/// 加密服务
pub struct EncryptionService;

impl EncryptionService {
    /// 创建加密服务实例
    pub fn new() -> Self {
        Self {}
    }
    
    /// 加密资源
    pub fn encrypt_resource(
        &self,
        data: &[u8],
        _media_type: &str,
        _is_local: bool,
        _key_part_a: &str,
        _ukey_part_b: &str,
        _config: &crate::config::AppConfig
    ) -> Result<(Vec<u8>, String), CryptoError> {
        // 这里需要实现实际的加密逻辑，目前还在开发中
        // 暂时返回原始数据和空的加密信息
        Ok((data.to_vec(), "{}".to_string()))
    }
    
    /// 解密资源
    pub fn decrypt_resource(
        &self,
        encrypted_data: &[u8],
        _encryption_info: &str,
        _key_part_a: &str,
        _ukey_part_b: &str
    ) -> Result<Vec<u8>, CryptoError> {
        // 这里需要实现实际的解密逻辑，目前还在开发中
        // 暂时返回原始数据
        Ok(encrypted_data.to_vec())
    }
    
    /// 验证密钥
    pub fn verify_resource_key(
        &self,
        key_part_a: &str,
        ukey_part_b: &str,
        encryption_info: &str
    ) -> Result<bool, CryptoError> {
        verify_key(key_part_a, ukey_part_b, encryption_info)
            .map_err(|e| CryptoError::Decode(e))
    }
}
