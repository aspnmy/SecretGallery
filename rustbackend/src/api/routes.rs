use axum::{Router, routing::get, routing::post, routing::put, routing::delete, Extension, http::StatusCode};
use std::sync::Arc;

use crate::config::AppConfig;
use crate::service::resource::ResourceService;
use crate::api::handlers::{resource_handlers, auth_handlers, health_handlers};

/// 404处理程序
async fn not_found_handler() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "404 Not Found - 密影库 后端服务")
}

/// 创建API路由
pub fn create_router(
    resource_service: Arc<ResourceService>,
    config: AppConfig
) -> Router {
    // 创建路由
    Router::new()
        // 健康检查
        .route("/health", get(health_handlers::health_check))
        
        // API路由组
        .nest("/api", {
            Router::new()
                // 资源管理
                .route("/resources", get(resource_handlers::get_resources))
                .route("/resources", post(resource_handlers::create_resource))
                .route("/resources/:id", get(resource_handlers::get_resource))
                .route("/resources/:id", put(resource_handlers::update_resource))
                .route("/resources/:id", delete(resource_handlers::delete_resource))
                .route("/resources/:id/decrypt", post(resource_handlers::decrypt_resource))
                .route("/resources/stats", get(resource_handlers::get_resource_stats))
                
                // 认证
                .route("/auth/login", post(auth_handlers::login))
                .route("/auth/register", post(auth_handlers::register))
                .route("/auth/verify", get(auth_handlers::verify))
                .route("/auth/logout", post(auth_handlers::logout))
        })
        
        // 添加404处理，使用axum::routing::any处理所有未匹配的请求
        .fallback(get(not_found_handler))
        
        // 添加中间件
        .layer(Extension(resource_service))
        .layer(Extension(config))
}
