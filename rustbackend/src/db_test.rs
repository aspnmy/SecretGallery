use sqlx::PgPool;

#[tokio::main]
async fn main() {
    // 直接使用编码后的URL测试
    let db_url = "postgresql://podadmin:X004252b%40%3D@10.168.3.165:65432/gocomicmosaic";
    
    println!("正在测试PostgreSQL数据库连接: {}", db_url);
    
    // 尝试连接数据库
    match PgPool::connect(db_url).await {
        Ok(pool) => {
            println!("数据库连接成功!");
            
            // 执行简单查询
            match sqlx::query("SELECT 1 as test").execute(&pool).await {
                Ok(_) => println!("数据库查询成功!") ,
                Err(e) => println!("数据库查询失败: {}", e),
            }
        },
        Err(e) => println!("数据库连接失败: {}", e),
    }
    
    println!("测试完成");
}