
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::database::schema::resource_status;

/// 资源模型
#[derive(FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct Resource {
    pub id: i32,
    pub title: String,
    pub title_en: Option<String>,
    pub description: Option<String>,
    pub resource_type: String,
    pub media_data: Vec<u8>,
    pub media_type: String,
    pub is_local: bool,
    pub encryption_info: String,
    pub status: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub total_count: Option<i32>,
    pub has_pending_supplement: Option<bool>,
}

/// 创建资源请求模型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateResourceRequest {
    pub title: String,
    pub title_en: Option<String>,
    pub description: String,
    pub resource_type: String,
    pub media_data: Vec<u8>,
    pub media_type: String,
    pub is_local: bool,
    pub encryption_info: String,
    pub status: Option<String>,
}

/// 更新资源请求模型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateResourceRequest {
    pub title: Option<String>,
    pub title_en: Option<String>,
    pub description: Option<String>,
    pub resource_type: Option<String>,
    pub media_data: Option<Vec<u8>>,
    pub media_type: Option<String>,
    pub is_local: Option<bool>,
    pub encryption_info: Option<String>,
    pub status: Option<String>,
}

/// 资源响应模型（不包含媒体数据）
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ResourceResponse {
    pub id: i32,
    pub title: String,
    pub title_en: String,
    pub description: String,
    pub resource_type: String,
    pub media_type: String,
    pub is_local: bool,
    pub status: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub total_count: Option<i32>,
    pub has_pending_supplement: Option<bool>,
}

/// 资源列表查询参数
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResourceQueryParams {
    pub skip: Option<u32>,
    pub limit: Option<u32>,
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub status: Option<String>,
    pub media_type: Option<String>,
    pub is_local: Option<bool>,
    pub count_only: Option<bool>,
    pub is_admin_view: Option<bool>,
    pub include_history: Option<bool>,
}

/// 资源统计信息
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ResourceStats {
    pub total: i32,
    pub pending: i32,
    pub approved: i32,
    pub rejected: i32,
    pub videos: i32,
    pub images: i32,
    pub local: i32,
    pub external: i32,
}

impl Resource {
    /// 创建新资源
    pub fn new(create_req: CreateResourceRequest) -> Self {
        let now = chrono::Utc::now().naive_utc();
        
        Self {
            id: 0,
            title: create_req.title,
            title_en: create_req.title_en,
            description: Some(create_req.description),
            resource_type: create_req.resource_type,
            media_data: create_req.media_data,
            media_type: create_req.media_type,
            is_local: create_req.is_local,
            encryption_info: create_req.encryption_info,
            status: create_req.status.unwrap_or(resource_status::PENDING.to_string()),
            created_at: now,
            updated_at: now,
            total_count: None,
            has_pending_supplement: None,
        }
    }
    
    /// 更新资源
    pub fn update(&mut self, update_req: UpdateResourceRequest) {
        if let Some(title) = update_req.title {
            self.title = title;
        }
        
        if let Some(title_en) = update_req.title_en {
            self.title_en = Some(title_en);
        }
        
        if let Some(description) = update_req.description {
            self.description = Some(description);
        }
        
        if let Some(resource_type) = update_req.resource_type {
            self.resource_type = resource_type;
        }
        
        if let Some(media_data) = update_req.media_data {
            self.media_data = media_data;
        }
        
        if let Some(media_type) = update_req.media_type {
            self.media_type = media_type;
        }
        
        if let Some(is_local) = update_req.is_local {
            self.is_local = is_local;
        }
        
        if let Some(encryption_info) = update_req.encryption_info {
            self.encryption_info = encryption_info;
        }
        
        if let Some(status) = update_req.status {
            self.status = status;
        }
        
        self.updated_at = chrono::Utc::now().naive_utc();
    }
    
    /// 转换为响应模型（不包含媒体数据）
    pub fn to_response(&self) -> ResourceResponse {
        ResourceResponse {
            id: self.id,
            title: self.title.clone(),
            title_en: self.title_en.clone().unwrap_or_default(),
            description: self.description.clone().unwrap_or_default(),
            resource_type: self.resource_type.clone(),
            media_type: self.media_type.clone(),
            is_local: self.is_local,
            status: self.status.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
            total_count: self.total_count,
            has_pending_supplement: self.has_pending_supplement,
        }
    }
    
    /// 检查资源是否已批准
    pub fn is_approved(&self) -> bool {
        self.status == resource_status::APPROVED
    }
    
    /// 检查资源是否待审批
    pub fn is_pending(&self) -> bool {
        self.status == resource_status::PENDING
    }
    
    /// 检查资源是否已拒绝
    pub fn is_rejected(&self) -> bool {
        self.status == resource_status::REJECTED
    }
    
    /// 检查资源是否为视频
    pub fn is_video(&self) -> bool {
        self.media_type == "video"
    }
    
    /// 检查资源是否为图片
    pub fn is_image(&self) -> bool {
        self.media_type == "image"
    }
    
    /// 检查资源是否为本地上传
    pub fn is_local_resource(&self) -> bool {
        self.is_local
    }
}

impl Default for ResourceQueryParams {
    fn default() -> Self {
        Self {
            skip: Some(0),
            limit: Some(100),
            search: None,
            sort_by: Some("created_at".to_string()),
            sort_order: Some("desc".to_string()),
            status: None,
            media_type: None,
            is_local: None,
            count_only: Some(false),
            is_admin_view: Some(false),
            include_history: Some(false),
        }
    }
}
