use std::env;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;

/// 从.env文件读取配置
fn read_env_file() -> std::collections::HashMap<String, String> {
    let mut env_vars = std::collections::HashMap::new();
    
    // 读取.env文件
    if let Ok(file) = fs::File::open(".env") {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                // 跳过注释和空行
                let trimmed = line.trim();
                if trimmed.starts_with('#') || trimmed.is_empty() {
                    continue;
                }
                
                // 解析键值对
                if let Some((key, value)) = trimmed.split_once('=') {
                    env_vars.insert(key.trim().to_string(), value.trim().to_string());
                }
            }
        }
    }
    
    env_vars
}

#[tokio::main]
async fn main() {
    println!("=== 数据库连接测试 ===\n");
    
    // 1. 读取.env文件内容
    println!("1. 读取.env文件内容:");
    let env_file_vars = read_env_file();
    if let Some(db_url) = env_file_vars.get("DATABASE_URL") {
        println!("DATABASE_URL: {}", db_url);
    }
    
    // 2. 使用从.env读取的URL进行测试
    println!("\n2. 使用从.env读取的URL进行测试:");
    // 从环境变量读取数据库URL
    let db_url = env::var("DATABASE_URL").unwrap_or_default();
    println!("使用的URL: {}", db_url);
    
    // 尝试连接数据库
    println!("\n正在连接到数据库...");
    match sqlx::PgPool::connect(&db_url).await {
        Ok(pool) => {
            println!("✅ 数据库连接成功！");
            
            // 测试查询
            match sqlx::query_as::<_, (i32,)>("SELECT 1 as test").fetch_one(&pool).await {
                Ok(result) => println!("✅ 查询测试成功，结果: {:?}", result),
                Err(e) => println!("❌ 查询测试失败: {:?}", e),
            }
        },
        Err(e) => {
            println!("❌ 数据库连接失败: {:?}", e);
            
            // 显示错误详情
            println!("\n错误详情:");
            println!("- 错误码: {:?}", e.as_database_error().map(|de| de.code()));
            println!("- 错误消息: {:?}", e.as_database_error().map(|de| de.message()));
        },
    }
    
    // 3. 显示系统环境变量（用于调试）
    println!("\n3. 系统环境变量（用于调试）:");
    println!("- PATH 长度: {:?}", env::var("PATH").map(|p| p.len()));
    println!("- HOME: {:?}", env::var("HOME").ok().map(|h| h.len()));
    println!("- USERNAME: {:?}", env::var("USERNAME"));
}