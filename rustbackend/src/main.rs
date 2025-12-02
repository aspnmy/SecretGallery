use std::net::SocketAddr;
use std::sync::Arc;

use axum::{serve};
use tracing::info;
use dotenvy::dotenv;

use crate::service::resource::ResourceService;

mod config;
mod database;
mod crypto;
mod service;
mod api;

#[tokio::main]
async fn main() {
    // 加载环境变量
    dotenv().ok();
    
    // 初始化日志
    tracing_subscriber::fmt::init();
    
    // 加载配置
    let config = config::AppConfig::from_env().expect("无法加载配置");
    config.validate().expect("配置验证失败");
    
    // 初始化数据库连接池
    let db_pool = database::init_database_pool(&config)
        .await
        .expect("无法初始化数据库连接池");
    
    // 创建服务实例
    let resource_service = Arc::new(ResourceService::new(db_pool.clone(), config.clone()));
    
    // 构建路由
    let app = api::routes::create_router(
        resource_service,
        config.clone()
    );
    
    // 配置服务器地址
    let addr = SocketAddr::from((
        [0, 0, 0, 0],
        config.server.port
    ));
    
    info!("服务器正在启动，监听地址: {}", addr);
    
    // 启动服务器
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("无法绑定地址");
    
    info!("服务器正在启动，监听地址: {}", listener.local_addr().unwrap());
    
    serve(listener, app)
        .await
        .expect("服务器启动失败");
}
