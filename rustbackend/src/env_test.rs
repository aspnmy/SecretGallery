use std::env;

fn main() {
    println!("=== 环境变量读取测试 ===\n");
    
    // 读取环境变量
    let dbman = env::var("dbman");
    let dbpwd = env::var("dbpwd");
    
    println!("环境变量 dbman 的值:");
    match dbman {
        Ok(value) => println!("✅ 成功读取: '{}'", value),
        Err(e) => println!("❌ 读取失败: {:?}", e),
    }
    
    println!("\n环境变量 dbpwd 的值:");
    match dbpwd {
        Ok(value) => println!("✅ 成功读取: '{}'", value),
        Err(e) => println!("❌ 读取失败: {:?}", e),
    }
    
    // 显示所有环境变量（可选，用于调试）
    println!("\n=== 所有环境变量列表（前20个） ===");
    let mut count = 0;
    for (key, value) in env::vars() {
        if count < 20 {
            println!("{}={}", key, value);
            count += 1;
        } else {
            break;
        }
    }
    println!("\n... 更多环境变量未显示");
}