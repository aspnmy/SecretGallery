use axum::{http::StatusCode, Json};
use chrono::{Utc, TimeZone, FixedOffset};

/// 健康检查响应
#[derive(serde::Serialize)]
pub struct HealthCheckResponse {
    /// 状态
    pub status: String,
    /// 时间戳（上海时区，24小时制）
    pub timestamp: String,
}

/// 健康检查处理器
pub async fn health_check() -> (StatusCode, Json<HealthCheckResponse>) {
    // 获取当前UTC时间
    let utc_now = Utc::now();
    
    // 创建上海时区偏移量 (UTC+8)
    let shanghai_offset = FixedOffset::east_opt(8 * 3600).unwrap();
    
    // 转换为上海时区
    let shanghai_time = shanghai_offset.from_utc_datetime(&utc_now.naive_utc());
    
    // 格式化为24小时制字符串（上海时区）
    let formatted_time = shanghai_time.format("%Y-%m-%d %H:%M:%S %Z").to_string();
    
    let response = HealthCheckResponse {
        status: "ok".to_string(),
        timestamp: formatted_time,
    };
    
    (StatusCode::OK, Json(response))
}
