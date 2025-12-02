use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// 加密密钥模型
#[derive(FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct EncryptionKey {
    pub id: i32,
    pub resource_id: i32,
    pub key_hash: String,
    pub ukey_info: String,
    pub created_at: chrono::NaiveDateTime,
}

/// 创建加密密钥请求模型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateEncryptionKeyRequest {
    pub resource_id: i32,
    pub key_hash: String,
    pub ukey_info: String,
}

/// 更新加密密钥请求模型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateEncryptionKeyRequest {
    pub key_hash: String,
    pub ukey_info: String,
}

/// 解密请求模型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DecryptRequest {
    pub key_part_a: String,
    pub ukey_part_b: String,
    pub resource_id: i32,
}

/// 解密响应模型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DecryptResponse {
    pub media_data: Vec<u8>,
    pub media_type: String,
    pub is_local: bool,
}

impl EncryptionKey {
    /// 创建新的加密密钥
    pub fn new(resource_id: i32, key_hash: String, ukey_info: String) -> Self {
        let now = chrono::Utc::now().naive_utc();
        
        Self {
            id: 0,
            resource_id,
            key_hash,
            ukey_info,
            created_at: now,
        }
    }
    
    /// 更新加密密钥
    pub fn update(&mut self, update_req: UpdateEncryptionKeyRequest) {
        self.key_hash = update_req.key_hash;
        self.ukey_info = update_req.ukey_info;
    }
    
    /// 验证密钥哈希
    pub fn verify_key_hash(&self, key_hash: &str) -> bool {
        self.key_hash == key_hash
    }
}
