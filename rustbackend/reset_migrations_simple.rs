use sqlx::PgPool;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 数据库连接信息
    let database_url = "postgresql://podadmin:X004252b%40%3D@10.168.3.165:65432/gocomicmosaic";
    
    println!("连接到数据库: {}", database_url);
    
    // 连接到数据库
    let pool = PgPool::connect(database_url).await?;
    
    println!("成功连接到数据库");
    
    // 删除_sqlx_migrations表（如果存在）
    let result = sqlx::query!("DROP TABLE IF EXISTS _sqlx_migrations CASCADE")
        .execute(&pool)
        .await;
    
    match result {
        Ok(_) => println!("成功删除_sqlx_migrations表"),
        Err(e) => {
            eprintln!("删除_sqlx_migrations表失败: {:?}", e);
            return Err(Box::new(e));
        }
    }
    
    println!("数据库迁移已重置，您现在可以重新运行迁移了");
    
    Ok(())
}
