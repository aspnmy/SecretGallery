use sqlx::{query, query_as};
use tracing::{info, error};
use anyhow::Result;

use crate::database::{DatabasePool, DatabaseError};
use crate::database::models::resource::{Resource, CreateResourceRequest, UpdateResourceRequest, ResourceQueryParams, ResourceStats, ResourceResponse};
use crate::database::models::encryption_key::{EncryptionKey, CreateEncryptionKeyRequest, UpdateEncryptionKeyRequest};
use crate::crypto::{encode_resource, decode_resource, verify_key, generate_key_hash, EncryptionService, EncodeError, DecodeError};
use crate::config::AppConfig;

/// 资源服务错误类型
#[derive(thiserror::Error, Debug)]
#[allow(dead_code)]
pub enum ResourceServiceError {
    #[error("数据库错误: {0}")]
    DatabaseError(#[from] DatabaseError),
    
    #[error("加密服务错误: {0}")]
    EncryptionError(#[from] crate::crypto::CryptoError),
    
    #[error("编码错误: {0}")]
    EncodeError(#[from] EncodeError),
    
    #[error("解码错误: {0}")]
    DecodeError(#[from] DecodeError),
    
    #[error("资源未找到")]
    ResourceNotFound,
    
    #[error("资源已存在")]
    ResourceExists,
    
    #[error("资源状态错误: {0}")]
    ResourceStatusError(String),
    
    #[error("参数错误: {0}")]
    ParameterError(String),
    
    #[error("权限错误: {0}")]
    PermissionError(String),
    
    #[error("解密失败: {0}")]
    DecryptionFailed(String),
    
    #[error("密钥验证失败")]
    KeyVerificationFailed,
    
    #[error("其他错误: {0}")]
    OtherError(#[from] anyhow::Error),
}

/// 资源服务
pub struct ResourceService {
  pub db: DatabasePool,
    pub config: AppConfig,
    #[allow(dead_code)]
    encryption_service: EncryptionService,
}

impl ResourceService {
    /// 创建资源服务实例
    pub fn new(db: DatabasePool, config: AppConfig) -> Self {
        Self {
            db,
            config,
            encryption_service: EncryptionService::new(),
        }
    }
    
    /// 获取资源列表
    pub async fn get_resources(&self, params: ResourceQueryParams) -> Result<(Vec<ResourceResponse>, Option<i32>), ResourceServiceError> {
        info!("获取资源列表，参数: {:?}", params);
        
        // 构建查询语句
        let mut base_query = String::from("SELECT * FROM resources");
        let mut count_query = String::from("SELECT COUNT(*) FROM resources");
        let mut conditions = Vec::new();
        let mut query_params = Vec::new();
        
        // 添加状态过滤
        if let Some(status) = params.status {
            conditions.push(format!("status = ?"));
            query_params.push(status);
        }
        
        // 添加媒体类型过滤
        if let Some(media_type) = params.media_type {
            conditions.push(format!("media_type = ?"));
            query_params.push(media_type);
        }
        
        // 添加本地资源过滤
        if let Some(is_local) = params.is_local {
            conditions.push(format!("is_local = ?"));
            query_params.push(is_local.to_string());
        }
        
        // 添加搜索条件
        if let Some(search) = params.search {
            conditions.push(format!("(title LIKE ? OR title_en LIKE ? OR description LIKE ?)",));
            let search_pattern = format!("%{}%", search);
            query_params.extend_from_slice(&[search_pattern.clone(), search_pattern.clone(), search_pattern]);
        }
        
        // 构建完整查询
        if !conditions.is_empty() {
            let where_clause = format!(" WHERE {}", conditions.join(" AND "));
            base_query += &where_clause;
            count_query += &where_clause;
        }
        
        // 添加排序
        let sort_by = params.sort_by.unwrap_or("created_at".to_string());
        let sort_order = params.sort_order.unwrap_or("desc".to_string());
        base_query += &format!(" ORDER BY {} {}", sort_by, sort_order);
        
        // 添加分页
        let skip = params.skip.unwrap_or(0);
        let limit = params.limit.unwrap_or(100);
        base_query += &format!(" LIMIT ? OFFSET ?");
        query_params.push(limit.to_string());
        query_params.push(skip.to_string());
        
        // 执行查询
        let mut query = query_as(&base_query);
        for param in query_params.clone() {
            query = query.bind(param);
        }
        let resources: Vec<Resource> = query.fetch_all(&self.db)
            .await
            .map_err(crate::database::DatabaseError::ConnectionError)?;
        
        // 获取总数（如果需要）
        let total_count = if params.count_only.unwrap_or(false) {
            let mut count_query = sqlx::query_scalar(&count_query);
            for param in query_params {
                count_query = count_query.bind(param);
            }
            let count: i32 = count_query.fetch_one(&self.db)
                .await
                .map_err(crate::database::DatabaseError::ConnectionError)?;
            Some(count)
        } else {
            None
        };
        
        // 转换为响应模型（不包含媒体数据）
        let resource_responses = resources
            .into_iter()
            .map(|resource| resource.to_response())
            .collect();
        
        Ok((resource_responses, total_count))
    }
    
    /// 根据ID获取资源
    pub async fn get_resource_by_id(&self, id: i32, is_admin_view: bool) -> Result<Resource, ResourceServiceError> {
        info!("根据ID获取资源: {}, 管理员视图: {}", id, is_admin_view);
        
        // 使用query!宏，手动处理类型转换
        let resource_row = sqlx::query!(r#"
            SELECT 
                id, 
                title, 
                title_en, 
                description, 
                resource_type, 
                media_data, 
                media_type, 
                is_local, 
                encryption_info, 
                status, 
                created_at, 
                updated_at, 
                total_count, 
                has_pending_supplement 
            FROM resources 
            WHERE id = $1
        "#, id)
        .fetch_optional(&self.db)
        .await
        .map_err(|e| ResourceServiceError::DatabaseError(DatabaseError::ConnectionError(e)))?
        .ok_or(ResourceServiceError::ResourceNotFound)?;
        
        // 手动构建Resource对象
        let resource = Resource {
            id: resource_row.id,
            title: resource_row.title,
            title_en: resource_row.title_en,
            description: resource_row.description,
            resource_type: resource_row.resource_type,
            media_data: resource_row.media_data,
            media_type: resource_row.media_type,
            is_local: resource_row.is_local,
            encryption_info: resource_row.encryption_info,
            status: resource_row.status,
            created_at: resource_row.created_at,
            updated_at: resource_row.updated_at,
            total_count: resource_row.total_count,
            has_pending_supplement: resource_row.has_pending_supplement,
        };
        
        // 如果不是管理员视图，只返回已批准的资源
        if !is_admin_view && resource.status != "APPROVED" {
            return Err(ResourceServiceError::ResourceStatusError("资源未批准".to_string()));
        }
        
        Ok(resource)
    }
    
    /// 创建资源
    pub async fn create_resource(
        &self,
        create_req: CreateResourceRequest,
        key_part_a: &str,
        ukey_part_b: &str
    ) -> Result<Resource, ResourceServiceError> {
        info!("创建资源: {}, 媒体类型: {}, 本地资源: {}", create_req.title, create_req.media_type, create_req.is_local);
        
        // 验证必填字段
        if create_req.title.is_empty() {
            return Err(ResourceServiceError::ParameterError("标题不能为空".to_string()));
        }
        
        if create_req.description.is_empty() {
            return Err(ResourceServiceError::ParameterError("描述不能为空".to_string()));
        }
        
        if create_req.resource_type.is_empty() {
            return Err(ResourceServiceError::ParameterError("资源类型不能为空".to_string()));
            
        }
        if create_req.media_type.is_empty() {
            return Err(ResourceServiceError::ParameterError("媒体类型不能为空".to_string()));
        }
        
        // 编码资源
        let encryption_info_json = encode_resource(
            &create_req.media_data,
            &create_req.media_type,
            create_req.is_local,
            key_part_a,
            ukey_part_b,
            &self.config
        )?;
        
        // 生成密钥哈希
        let actual_key = format!("{}{}", key_part_a, ukey_part_b);
        let key_hash = generate_key_hash(&actual_key);
        
        // 创建资源记录
        let now = chrono::Utc::now().naive_utc();
        let status = create_req.status.unwrap_or("PENDING".to_string());
        
        let resource_id = query!(r#"INSERT INTO resources 
            (title, title_en, description, resource_type, media_data, media_type, is_local, encryption_info, status, created_at, updated_at) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) 
            RETURNING id"#, 
            create_req.title, 
            create_req.title_en.unwrap_or_default(), 
            create_req.description, 
            create_req.resource_type, 
            create_req.media_data, 
            create_req.media_type, 
            create_req.is_local, 
            encryption_info_json, 
            status, 
            now, 
            now
        )
        .fetch_one(&self.db)
        .await
        .map_err(|e| ResourceServiceError::DatabaseError(DatabaseError::ConnectionError(e)))?
        .id;
        
        // 创建加密密钥记录
        let encryption_key = CreateEncryptionKeyRequest {
            resource_id,
            key_hash,
            ukey_info: "".to_string(), // 这里应该存储UKey的相关信息
        };
        
        self.create_encryption_key(&encryption_key).await?;
        
        // 获取创建的资源
        let resource = self.get_resource_by_id(resource_id, true).await?;
        
        info!("资源创建成功: {}, ID: {}", create_req.title, resource_id);
        
        Ok(resource)
    }
    
    /// 更新资源
    pub async fn update_resource(
        &self,
        id: i32,
        update_req: UpdateResourceRequest
    ) -> Result<Resource, ResourceServiceError> {
        info!("更新资源: {}", id);
        
        // 检查资源是否存在
        let mut resource = self.get_resource_by_id(id, true).await?;
        
        // 更新资源字段
        resource.update(update_req);
        
        // 执行更新
        query!(r#"UPDATE resources SET 
            title = $1, title_en = $2, description = $3, resource_type = $4, media_data = $5, media_type = $6, is_local = $7, encryption_info = $8, status = $9, updated_at = $10 
            WHERE id = $11"#, 
            resource.title, 
            resource.title_en, 
            resource.description, 
            resource.resource_type, 
            resource.media_data, 
            resource.media_type, 
            resource.is_local, 
            resource.encryption_info, 
            resource.status, 
            resource.updated_at, 
            id
        )
        .execute(&self.db)
        .await
        .map_err(|e| ResourceServiceError::DatabaseError(DatabaseError::ConnectionError(e)))?;
        
        info!("资源更新成功: {}", id);
        
        Ok(resource)
    }
    
    /// 删除资源
    pub async fn delete_resource(&self, id: i32) -> Result<(), ResourceServiceError> {
        info!("删除资源: {}", id);
        
        // 检查资源是否存在
        let _ = self.get_resource_by_id(id, true).await?;
        
        // 开始事务
        let mut transaction = self.db.begin()
            .await
            .map_err(|e| ResourceServiceError::DatabaseError(DatabaseError::ConnectionError(e)))?;
        
        // 删除加密密钥
        sqlx::query!("DELETE FROM encryption_keys WHERE resource_id = $1", id)
            .execute(&mut *transaction)
            .await
            .map_err(|e| ResourceServiceError::DatabaseError(DatabaseError::ConnectionError(e)))?;
        
        // 删除资源
        sqlx::query!("DELETE FROM resources WHERE id = $1", id)
            .execute(&mut *transaction)
            .await
            .map_err(|e| ResourceServiceError::DatabaseError(DatabaseError::ConnectionError(e)))?;
        
        // 提交事务
        transaction.commit()
            .await
            .map_err(|e| ResourceServiceError::DatabaseError(DatabaseError::ConnectionError(e)))?;
        
        info!("资源删除成功: {}", id);
        
        Ok(())
    }
    
    /// 解密资源
    pub async fn decrypt_resource(
        &self,
        id: i32,
        key_part_a: &str,
        ukey_part_b: &str
    ) -> Result<Vec<u8>, ResourceServiceError> {
        info!("解密资源: {}", id);
        
        // 获取资源
        let resource = self.get_resource_by_id(id, true).await?;
        
        // 验证密钥
        if !verify_key(key_part_a, ukey_part_b, &resource.encryption_info)? {
            return Err(ResourceServiceError::KeyVerificationFailed);
        }
        
        // 解密资源
        let decrypted_data = decode_resource(
            &resource.media_data,
            &resource.encryption_info,
            key_part_a,
            ukey_part_b
        )?;
        
        info!("资源解密成功: {}", id);
        
        Ok(decrypted_data)
    }
    
    /// 获取资源统计信息
    pub async fn get_resource_stats(&self) -> Result<ResourceStats, ResourceServiceError> {
        info!("获取资源统计信息");
        
        // 总资源数
        let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM resources")
            .fetch_one(&self.db)
            .await
            .map_err(|e| ResourceServiceError::DatabaseError(DatabaseError::ConnectionError(e)))?;
        
        // 待审批资源数
        let pending: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM resources WHERE status = $1")
            .bind("PENDING")
            .fetch_one(&self.db)
            .await
            .map_err(|e| ResourceServiceError::DatabaseError(DatabaseError::ConnectionError(e)))?;
        
        // 已批准资源数
        let approved: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM resources WHERE status = $1")
            .bind("APPROVED")
            .fetch_one(&self.db)
            .await
            .map_err(|e| ResourceServiceError::DatabaseError(DatabaseError::ConnectionError(e)))?;
        
        // 已拒绝资源数
        let rejected: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM resources WHERE status = $1")
            .bind("REJECTED")
            .fetch_one(&self.db)
            .await
            .map_err(|e| ResourceServiceError::DatabaseError(DatabaseError::ConnectionError(e)))?;
        
        // 视频资源数
        let videos: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM resources WHERE media_type = $1")
            .bind("video")
            .fetch_one(&self.db)
            .await
            .map_err(|e| ResourceServiceError::DatabaseError(DatabaseError::ConnectionError(e)))?;
        
        // 图片资源数
        let images: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM resources WHERE media_type = $1")
            .bind("image")
            .fetch_one(&self.db)
            .await
            .map_err(|e| ResourceServiceError::DatabaseError(DatabaseError::ConnectionError(e)))?;
        
        // 本地资源数
        let local: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM resources WHERE is_local = $1")
            .bind(true)
            .fetch_one(&self.db)
            .await
            .map_err(|e| ResourceServiceError::DatabaseError(DatabaseError::ConnectionError(e)))?;
        
        // 外部资源数
        let external: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM resources WHERE is_local = $1")
            .bind(false)
            .fetch_one(&self.db)
            .await
            .map_err(|e| ResourceServiceError::DatabaseError(DatabaseError::ConnectionError(e)))?;
        
        let stats = ResourceStats {
            total: total as i32,
            pending: pending as i32,
            approved: approved as i32,
            rejected: rejected as i32,
            videos: videos as i32,
            images: images as i32,
            local: local as i32,
            external: external as i32,
        };
        
        info!("资源统计信息: {:?}", stats);
        
        Ok(stats)
    }
    
    /// 创建加密密钥
    async fn create_encryption_key(
        &self,
        create_req: &CreateEncryptionKeyRequest
    ) -> Result<EncryptionKey, ResourceServiceError> {
        let now = chrono::Utc::now().naive_utc();
        
        let encryption_key_id = query!(r#"INSERT INTO encryption_keys 
            (resource_id, key_hash, ukey_info, created_at) 
            VALUES ($1, $2, $3, $4) 
            RETURNING id"#, 
            create_req.resource_id, 
            create_req.key_hash, 
            create_req.ukey_info, 
            now
        )
        .fetch_one(&self.db)
        .await
        .map_err(|e| ResourceServiceError::DatabaseError(DatabaseError::ConnectionError(e)))?
        .id;
        
        // 获取创建的加密密钥
        let encryption_key = query_as!(EncryptionKey, "SELECT * FROM encryption_keys WHERE id = $1", encryption_key_id)
            .fetch_one(&self.db)
            .await
            .map_err(|e| ResourceServiceError::DatabaseError(DatabaseError::ConnectionError(e)))?;
        
        Ok(encryption_key)
    }
    
    /// 更新加密密钥
    #[allow(dead_code)]
    async fn update_encryption_key(
          &self,
          resource_id: i32,
          update_req: &UpdateEncryptionKeyRequest
      ) -> Result<EncryptionKey, ResourceServiceError> {
        let encryption_key = query_as!(EncryptionKey, "SELECT * FROM encryption_keys WHERE resource_id = $1", resource_id)
            .fetch_optional(&self.db)
            .await
            .map_err(|e| ResourceServiceError::DatabaseError(DatabaseError::ConnectionError(e)))?
            .ok_or(ResourceServiceError::ResourceNotFound)?;
        
        query!(r#"UPDATE encryption_keys SET 
            key_hash = $1, ukey_info = $2 
            WHERE id = $3"#, 
            update_req.key_hash, 
            update_req.ukey_info, 
            encryption_key.id
        )
        .execute(&self.db)
        .await
        .map_err(|e| ResourceServiceError::DatabaseError(DatabaseError::ConnectionError(e)))?;
        
        // 获取更新后的加密密钥
        let updated_encryption_key = query_as!(EncryptionKey, "SELECT * FROM encryption_keys WHERE id = $1", encryption_key.id)
            .fetch_one(&self.db)
            .await
            .map_err(|e| ResourceServiceError::DatabaseError(DatabaseError::ConnectionError(e)))?;
        
        Ok(updated_encryption_key)
    }
    
    /// 获取加密密钥
    #[allow(dead_code)]
    async fn get_encryption_key(
          &self,
          resource_id: i32
      ) -> Result<EncryptionKey, ResourceServiceError> {
        let encryption_key = query_as!(EncryptionKey, "SELECT * FROM encryption_keys WHERE resource_id = $1", resource_id)
            .fetch_optional(&self.db)
            .await
            .map_err(|e| ResourceServiceError::DatabaseError(DatabaseError::ConnectionError(e)))?
            .ok_or(ResourceServiceError::ResourceNotFound)?;
        
        Ok(encryption_key)
    }
}
