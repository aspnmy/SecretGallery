use sqlx::PgPool;
use dotenvy::dotenv;
use std::env;
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // 加载环境变量
    dotenv().ok();
    
    // 获取数据库URL
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    info!("连接到数据库: {}", database_url);
    
    // 连接到数据库
    let pool = PgPool::connect(&database_url).await?;
    
    info!("成功连接到数据库");
    
    // 删除_sqlx_migrations表（如果存在）
    let result = sqlx::query!("DROP TABLE IF EXISTS _sqlx_migrations CASCADE")
        .execute(&pool)
        .await;
    
    match result {
        Ok(_) => info!("成功删除_sqlx_migrations表"),
        Err(e) => {
            error!("删除_sqlx_migrations表失败: {:?}", e);
            return Err(e);
        }
    }
    
    info!("数据库迁移已重置，您现在可以重新运行迁移了");
    
    Ok(())
}
