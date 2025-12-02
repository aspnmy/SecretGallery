use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// 用户模型
#[derive(FromRow, Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub hashed_password: String,
    pub is_admin: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 创建用户请求模型
#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
    pub is_admin: Option<bool>,
}

/// 更新用户请求模型
#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub password: Option<String>,
    pub is_admin: Option<bool>,
}

/// 用户响应模型（不包含密码哈希）
#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub is_admin: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 登录请求模型
#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// 登录响应模型
#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
pub struct LoginResponse {
    pub access_token: String,
    pub token_type: String,
    pub user: UserResponse,
}

/// 密码更新请求模型
#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
pub struct PasswordUpdateRequest {
    pub current_password: String,
    pub new_password: String,
}

#[allow(dead_code)]
impl User {
    /// 创建新用户
    pub fn new(username: String, hashed_password: String, is_admin: bool) -> Self {
        let now = Utc::now();
        
        Self {
            id: 0,
            username,
            hashed_password,
            is_admin,
            created_at: now,
            updated_at: now,
        }
    }
    
    /// 更新用户
    pub fn update(&mut self, update_req: UpdateUserRequest, hashed_password: Option<String>) {
        if let Some(username) = update_req.username {
            self.username = username;
        }
        
        if let Some(password) = hashed_password {
            self.hashed_password = password;
        }
        
        if let Some(is_admin) = update_req.is_admin {
            self.is_admin = is_admin;
        }
        
        self.updated_at = Utc::now();
    }
    
    /// 转换为响应模型（不包含密码哈希）
    pub fn to_response(&self) -> UserResponse {
        UserResponse {
            id: self.id,
            username: self.username.clone(),
            is_admin: self.is_admin,
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
    
    /// 检查用户是否为管理员
    pub fn is_admin(&self) -> bool {
        self.is_admin
    }
}
