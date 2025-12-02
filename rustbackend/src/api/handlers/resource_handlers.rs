use axum::{http::StatusCode, Json, Extension};
use axum::extract::{Path, Query};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use base64::{Engine as _, engine::general_purpose};

use crate::service::resource::ResourceService;
use crate::database::models::{CreateResourceRequest, UpdateResourceRequest, ResourceQueryParams, ResourceStats, ResourceResponse};
use crate::service::resource::ResourceServiceError;

/// 资源列表查询参数
#[derive(Deserialize, Debug)]
pub struct ResourceListQuery {
    /// 跳过的记录数
    pub skip: Option<i32>,
    /// 返回的记录数
    pub limit: Option<i32>,
    /// 状态过滤
    pub status: Option<String>,
    /// 媒体类型过滤
    pub media_type: Option<String>,
    /// 是否本地资源过滤
    pub is_local: Option<bool>,
    /// 搜索关键词
    pub search: Option<String>,
    /// 排序字段
    pub sort_by: Option<String>,
    /// 排序顺序
    pub sort_order: Option<String>,
    /// 是否只返回总数
    pub count_only: Option<bool>,
}

/// 资源解密请求
#[derive(Deserialize, Debug)]
pub struct DecryptResourceRequest {
    /// 密钥部分A（用户密钥）
    pub key_part_a: String,
    /// 密钥部分B（硬件UKey）
    pub ukey_part_b: String,
}

/// 资源创建响应
#[derive(Serialize, Debug)]
pub struct ResourceCreateResponse {
    /// 资源
    pub resource: ResourceResponse,
    /// 消息
    pub message: String,
}

/// 资源更新响应
#[derive(Serialize, Debug)]
pub struct ResourceUpdateResponse {
    /// 资源
    pub resource: ResourceResponse,
    /// 消息
    pub message: String,
}

/// 资源删除响应
#[derive(Serialize, Debug)]
pub struct ResourceDeleteResponse {
    /// 消息
    pub message: String,
}

/// 资源解密响应
#[derive(Serialize, Debug)]
pub struct ResourceDecryptResponse {
    /// 解密后的数据（Base64编码）
    pub data: String,
    /// 消息
    pub message: String,
}

/// 获取资源列表
#[axum::debug_handler]
pub async fn get_resources(
    Query(query): Query<ResourceListQuery>,
    Extension(resource_service): Extension<Arc<ResourceService>>,
) -> (StatusCode, Json<Vec<ResourceResponse>>) {
    let params = ResourceQueryParams {
        skip: query.skip.map(|x| x as u32),
        limit: query.limit.map(|x| x as u32),
        status: query.status,
        media_type: query.media_type,
        is_local: query.is_local,
        search: query.search,
        sort_by: query.sort_by,
        sort_order: query.sort_order,
        count_only: query.count_only,
        is_admin_view: None,
        include_history: None,
    };
    
    match resource_service.get_resources(params).await {
        Ok((resources, _)) => (StatusCode::OK, Json(resources)),
        Err(err) => {
            tracing::error!("获取资源列表失败: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![]))
        }
    }
}

/// 获取单个资源
#[axum::debug_handler]
pub async fn get_resource(
    Path(id): Path<i32>,
    Extension(resource_service): Extension<Arc<ResourceService>>,
) -> (StatusCode, Json<ResourceResponse>) {
    match resource_service.get_resource_by_id(id, false).await {
        Ok(resource) => (StatusCode::OK, Json(resource.to_response())),
        Err(ResourceServiceError::ResourceNotFound) => (StatusCode::NOT_FOUND, Json(ResourceResponse::default())),
        Err(ResourceServiceError::ResourceStatusError(_)) => (StatusCode::FORBIDDEN, Json(ResourceResponse::default())),
        Err(err) => {
            tracing::error!("获取资源失败: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ResourceResponse::default()))
        }
    }
}

/// 创建资源
#[axum::debug_handler]
pub async fn create_resource(
    Extension(resource_service): Extension<Arc<ResourceService>>,
    Json(req): Json<CreateResourceRequest>,
) -> (StatusCode, Json<ResourceCreateResponse>) {
    // 这里需要从请求头或会话中获取key_part_a和ukey_part_b
    // 暂时使用默认值，实际应用中需要从认证系统中获取
    let key_part_a = "default_key_part_a"; // 实际应从认证系统获取
    let ukey_part_b = "default_ukey_part_b"; // 实际应从UKey系统获取
    
    match resource_service.create_resource(req, key_part_a, ukey_part_b).await {
        Ok(resource) => {
            let response = ResourceCreateResponse {
                resource: resource.to_response(),
                message: "资源创建成功".to_string(),
            };
            (StatusCode::CREATED, Json(response))
        },
        Err(err) => {
            tracing::error!("创建资源失败: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ResourceCreateResponse {
                resource: ResourceResponse::default(),
                message: format!("资源创建失败: {:?}", err),
            }))
        }
    }
}

/// 更新资源
#[axum::debug_handler]
pub async fn update_resource(
    Path(id): Path<i32>,
    Extension(resource_service): Extension<Arc<ResourceService>>,
    Json(req): Json<UpdateResourceRequest>,
) -> (StatusCode, Json<ResourceUpdateResponse>) {
    match resource_service.update_resource(id, req).await {
        Ok(resource) => {
            let response = ResourceUpdateResponse {
                resource: resource.to_response(),
                message: "资源更新成功".to_string(),
            };
            (StatusCode::OK, Json(response))
        },
        Err(ResourceServiceError::ResourceNotFound) => (StatusCode::NOT_FOUND, Json(ResourceUpdateResponse {
            resource: ResourceResponse::default(),
            message: "资源未找到".to_string(),
        })),
        Err(err) => {
            tracing::error!("更新资源失败: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ResourceUpdateResponse {
                resource: ResourceResponse::default(),
                message: format!("资源更新失败: {:?}", err),
            }))
        }
    }
}

/// 删除资源
#[axum::debug_handler]
pub async fn delete_resource(
    Path(id): Path<i32>,
    Extension(resource_service): Extension<Arc<ResourceService>>,
) -> (StatusCode, Json<ResourceDeleteResponse>) {
    match resource_service.delete_resource(id).await {
        Ok(_) => {
            let response = ResourceDeleteResponse {
                message: "资源删除成功".to_string(),
            };
            (StatusCode::OK, Json(response))
        },
        Err(ResourceServiceError::ResourceNotFound) => (StatusCode::NOT_FOUND, Json(ResourceDeleteResponse {
            message: "资源未找到".to_string(),
        })),
        Err(err) => {
            tracing::error!("删除资源失败: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ResourceDeleteResponse {
                message: format!("资源删除失败: {:?}", err),
            }))
        }
    }
}

/// 解密资源
#[axum::debug_handler]
pub async fn decrypt_resource(
    Path(id): Path<i32>,
    Extension(resource_service): Extension<Arc<ResourceService>>,
    Json(req): Json<DecryptResourceRequest>,
) -> (StatusCode, Json<ResourceDecryptResponse>) {
    match resource_service.decrypt_resource(id, &req.key_part_a, &req.ukey_part_b).await {
        Ok(data) => {
            let response = ResourceDecryptResponse {
                data: general_purpose::STANDARD.encode(data),
                message: "资源解密成功".to_string(),
            };
            (StatusCode::OK, Json(response))
        },
        Err(ResourceServiceError::ResourceNotFound) => (StatusCode::NOT_FOUND, Json(ResourceDecryptResponse {
            data: String::new(),
            message: "资源未找到".to_string(),
        })),
        Err(ResourceServiceError::KeyVerificationFailed) => (StatusCode::UNAUTHORIZED, Json(ResourceDecryptResponse {
            data: String::new(),
            message: "密钥验证失败".to_string(),
        })),
        Err(err) => {
            tracing::error!("解密资源失败: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ResourceDecryptResponse {
                data: String::new(),
                message: format!("资源解密失败: {:?}", err),
            }))
        }
    }
}

/// 获取资源统计信息
#[axum::debug_handler]
pub async fn get_resource_stats(
    Extension(resource_service): Extension<Arc<ResourceService>>,
) -> (StatusCode, Json<ResourceStats>) {
    match resource_service.get_resource_stats().await {
        Ok(stats) => (StatusCode::OK, Json(stats)),
        Err(err) => {
            tracing::error!("获取资源统计信息失败: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ResourceStats::default()))
        }
    }
}
