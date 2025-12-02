use sqlx::{PgPool, Pool, Postgres};
use tracing::{info, error};

use crate::config::AppConfig;

/// 数据库错误类型
#[derive(thiserror::Error, Debug)]
pub enum DatabaseError {
    #[error("数据库连接错误: {0}")]
    ConnectionError(#[from] sqlx::Error),
    
    #[error("数据库迁移错误: {0}")]
    MigrationError(String),
    
    #[error("数据库查询错误: {0}")]
    QueryError(#[from] anyhow::Error),
    
    #[error("数据库事务错误: {0}")]
    TransactionError(String),
    
    #[error("数据库记录未找到")]
    RecordNotFound,
    
    #[error("数据库约束违反: {0}")]
    ConstraintViolation(String),
}

/// PostgreSQL数据库连接池
pub type DatabasePool = Pool<Postgres>;

/// 初始化数据库连接池
pub async fn init_database_pool(config: &AppConfig) -> Result<DatabasePool, DatabaseError> {
    info!("正在连接PostgreSQL数据库: {}", config.database.url);
    
    let pool = PgPool::connect(&config.database.url)
        .await?;
    
    info!("PostgreSQL数据库连接池初始化成功，最大连接数: {}", config.database.pool_size);
    
    // 执行数据库迁移
    if let Err(e) = run_migrations(&pool).await {
        error!("数据库迁移失败: {}", e);
        return Err(DatabaseError::MigrationError(e.to_string()));
    }
    
    info!("数据库迁移执行成功");
    
    Ok(pool)
}

/// 执行数据库迁移
async fn run_migrations(pool: &DatabasePool) -> Result<(), sqlx::Error> {
    // 执行表创建语句
    sqlx::migrate!("./migrations").run(pool).await?;
    
    Ok(())
}

/// 检查数据库连接是否正常
pub async fn check_database_connection(pool: &DatabasePool) -> Result<(), DatabaseError> {
    sqlx::query("SELECT 1")
        .execute(pool)
        .await?;
    
    Ok(())
}

/// 获取数据库版本
pub async fn get_database_version(pool: &DatabasePool) -> Result<String, DatabaseError> {
    let version: String = sqlx::query_scalar("SELECT version();")
        .fetch_one(pool)
        .await?;
    
    Ok(version)
}

// 导出子模块
pub mod models;
pub mod schema;
