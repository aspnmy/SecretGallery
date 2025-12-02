use sqlx::PgPool;

#[tokio::main]
async fn main() {
    println!("=== 简单数据库连接测试 ===\n");
    
    // 直接使用完整的数据库连接URL
    let db_url = "postgresql://podadmin:X004252b%40%3D@10.168.3.165:65432/gocomicmosaic";
    println!("使用的数据库URL: {}", db_url);
    
    // 尝试连接数据库
    println!("\n正在尝试连接数据库...");
    match PgPool::connect(db_url).await {
        Ok(pool) => {
            println!("✅ 数据库连接成功！");
            
            // 测试查询
            println!("\n正在执行测试查询...");
            match sqlx::query_as::<_, (i32,)>("SELECT 1 as test").fetch_one(&pool).await {
                Ok(result) => println!("✅ 查询测试成功！结果: {:?}", result),
                Err(e) => println!("❌ 查询测试失败: {:?}", e),
            }
        },
        Err(e) => {
            println!("❌ 数据库连接失败！");
            println!("详细错误: {:?}", e);
            
            // 分析错误类型
            match e {
                sqlx::Error::Configuration(_) => {
                    println!("\n❌ 配置错误：请检查数据库URL格式是否正确");
                },
                sqlx::Error::Database(db_err) => {
                    println!("\n❌ 数据库错误：");
                    println!("  - 错误码: {:?}", db_err.code());
                    println!("  - 错误消息: {:?}", db_err.message());
                },
                sqlx::Error::Io(io_err) => {
                    println!("\n❌ 网络/IO错误：");
                    println!("  - 错误: {:?}", io_err);
                    println!("  - 请检查数据库服务器是否运行，以及网络连接是否正常");
                },
                _ => {
                    println!("\n❌ 其他错误: {:?}", e);
                }
            }
        },
    }
    
    println!("\n=== 测试结束 ===");
}