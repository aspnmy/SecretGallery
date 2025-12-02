use sqlx::migrate::Migrator;

// 数据库迁移器
pub static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

/// 数据库表名常量
pub mod table_name {
    pub const RESOURCES: &str = "resources";
    pub const USERS: &str = "users";
    pub const APPROVALS: &str = "approvals";
    pub const ENCRYPTION_KEYS: &str = "encryption_keys";
    pub const SITE_SETTINGS: &str = "site_settings";
    pub const LIKES: &str = "likes";
    pub const STICKERS: &str = "stickers";
    pub const SUPPLEMENTS: &str = "supplements";
}

/// 资源状态枚举
pub mod resource_status {
    pub const PENDING: &str = "PENDING";
    pub const APPROVED: &str = "APPROVED";
    pub const REJECTED: &str = "REJECTED";
}

/// 媒体类型枚举
pub mod media_type {
    pub const VIDEO: &str = "video";
    pub const IMAGE: &str = "image";
}

/// 加密算法枚举
pub mod encryption_algorithm {
    pub const AES256GCM: &str = "AES256GCM";
    pub const CHACHA20POLY1305: &str = "CHACHA20POLY1305";
}

/// SQL查询常量
pub mod query {
    // 资源查询
    pub const GET_RESOURCES: &str = "SELECT * FROM resources";
    pub const GET_RESOURCE_BY_ID: &str = "SELECT * FROM resources WHERE id = ?";
    pub const CREATE_RESOURCE: &str = "INSERT INTO resources (title, title_en, description, resource_type, media_data, media_type, is_local, encryption_info, status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";
    pub const UPDATE_RESOURCE: &str = "UPDATE resources SET title = ?, title_en = ?, description = ?, resource_type = ?, media_data = ?, media_type = ?, is_local = ?, encryption_info = ?, status = ?, updated_at = ? WHERE id = ?";
    pub const DELETE_RESOURCE: &str = "DELETE FROM resources WHERE id = ?";
    pub const GET_RESOURCE_COUNT: &str = "SELECT COUNT(*) FROM resources";
    
    // 用户查询
    pub const GET_USER_BY_ID: &str = "SELECT * FROM users WHERE id = ?";
    pub const GET_USER_BY_USERNAME: &str = "SELECT * FROM users WHERE username = ?";
    pub const CREATE_USER: &str = "INSERT INTO users (username, hashed_password, is_admin, created_at, updated_at) VALUES (?, ?, ?, ?, ?)";
    pub const UPDATE_USER: &str = "UPDATE users SET username = ?, hashed_password = ?, is_admin = ?, updated_at = ? WHERE id = ?";
    pub const DELETE_USER: &str = "DELETE FROM users WHERE id = ?";
    
    // 加密密钥查询
    pub const GET_ENCRYPTION_KEY: &str = "SELECT * FROM encryption_keys WHERE resource_id = ?";
    pub const CREATE_ENCRYPTION_KEY: &str = "INSERT INTO encryption_keys (resource_id, key_hash, ukey_info, created_at) VALUES (?, ?, ?, ?)";
    pub const UPDATE_ENCRYPTION_KEY: &str = "UPDATE encryption_keys SET key_hash = ?, ukey_info = ? WHERE resource_id = ?";
    pub const DELETE_ENCRYPTION_KEY: &str = "DELETE FROM encryption_keys WHERE resource_id = ?";
    
    // 审批查询
    pub const GET_APPROVALS: &str = "SELECT * FROM approvals WHERE resource_id = ?";
    pub const CREATE_APPROVAL: &str = "INSERT INTO approvals (resource_id, status, field_approvals, field_rejections, approved_images, rejected_images, poster_image, notes, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";
    pub const UPDATE_APPROVAL: &str = "UPDATE approvals SET status = ?, field_approvals = ?, field_rejections = ?, approved_images = ?, rejected_images = ?, poster_image = ?, notes = ?, updated_at = ? WHERE id = ?";
    
    // 站点设置查询
    pub const GET_SETTING: &str = "SELECT * FROM site_settings WHERE setting_key = ?";
    pub const UPDATE_SETTING: &str = "INSERT OR REPLACE INTO site_settings (setting_key, setting_value, created_at, updated_at) VALUES (?, ?, ?, ?)";
    
    // 点赞查询
    pub const GET_LIKE: &str = "SELECT * FROM likes WHERE resource_id = ? AND user_id = ?";
    pub const CREATE_LIKE: &str = "INSERT INTO likes (resource_id, user_id, created_at) VALUES (?, ?, ?)";
    pub const DELETE_LIKE: &str = "DELETE FROM likes WHERE resource_id = ? AND user_id = ?";
    pub const GET_LIKE_COUNT: &str = "SELECT COUNT(*) FROM likes WHERE resource_id = ?";
}
