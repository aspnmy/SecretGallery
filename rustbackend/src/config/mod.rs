use serde::Deserialize;
use std::env;
use std::path::Path;

/// 配置错误类型
#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error("环境变量 {0} 未设置")]
    MissingEnvVar(String),
    
    #[error("环境变量 {0} 解析失败: {1}")]
    ParseError(String, String),
    
    #[error("无法加载环境变量文件: {0}")]
    LoadError(#[from] dotenvy::Error),
    
    #[error("其他配置错误: {0}")]
    Other(#[from] anyhow::Error),
}

/// 服务器配置
#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct ServerConfig {
    pub port: u16,
    pub host: String,
    pub env: String,
}

/// 数据库配置
#[derive(Deserialize, Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub pool_size: u32,
}

/// JWT配置
#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration: u64,
}

/// 加密配置
#[derive(Deserialize, Debug, Clone)]
pub struct EncryptionConfig {
    pub algorithm: String,
    pub salt: String,
    pub key_derivation_iterations: u32,
}

/// UKey配置
#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct UKeyConfig {
    pub vendor: String,
    pub api_url: String,
}

/// TMDB配置
#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct TmdbConfig {
    pub api_key: String,
    pub api_url: String,
    pub enabled: bool,
}

/// 图片处理配置
#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct ImageConfig {
    pub compression_quality: u8,
    pub max_width: u32,
    pub max_height: u32,
}

/// 日志配置
#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct LogConfig {
    pub level: String,
    pub file: String,
}

/// CORS配置
#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct CorsConfig {
    pub allow_origins: String,
    pub allow_methods: String,
    pub allow_headers: String,
}

/// 上传配置
#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct UploadConfig {
    pub max_size: u64,
    pub temp_dir: String,
}

/// 应用配置
#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub jwt: JwtConfig,
    pub encryption: EncryptionConfig,
    pub ukey: UKeyConfig,
    pub tmdb: TmdbConfig,
    pub image: ImageConfig,
    pub log: LogConfig,
    pub cors: CorsConfig,
    pub upload: UploadConfig,
}

impl AppConfig {
    /// 从环境变量加载配置
    pub fn from_env() -> Result<Self, ConfigError> {
        // 尝试加载.env文件，如果存在的话
        let _ = dotenvy::from_path(".env");
        
        Ok(Self {
            server: ServerConfig {
                port: get_env_var("PORT").map_or("8000".to_string(), |v| v).parse::<u16>().map_err(|e| ConfigError::ParseError("PORT".to_string(), e.to_string()))?,
                host: get_env_var("HOST").map_or("0.0.0.0".to_string(), |v| v),
                env: get_env_var("ENV").map_or("development".to_string(), |v| v),
            },
            database: DatabaseConfig {
                url: get_env_var("DATABASE_URL").map_or("sqlite:./gcm.db".to_string(), |v| v),
                pool_size: get_env_var("DATABASE_POOL_SIZE").map_or("10".to_string(), |v| v).parse::<u32>().map_err(|e| ConfigError::ParseError("DATABASE_POOL_SIZE".to_string(), e.to_string()))?,
            },
            jwt: JwtConfig {
                secret: get_env_var("JWT_SECRET").ok_or(ConfigError::MissingEnvVar("JWT_SECRET".to_string()))?,
                expiration: get_env_var("JWT_EXPIRATION").map_or("3600".to_string(), |v| v).parse::<u64>().map_err(|e| ConfigError::ParseError("JWT_EXPIRATION".to_string(), e.to_string()))?,
            },
            encryption: EncryptionConfig {
                algorithm: get_env_var("ENCRYPTION_ALGORITHM").map_or("AES256GCM".to_string(), |v| v),
                salt: get_env_var("ENCRYPTION_SALT").ok_or(ConfigError::MissingEnvVar("ENCRYPTION_SALT".to_string()))?,
                key_derivation_iterations: get_env_var("KEY_DERIVATION_ITERATIONS").map_or("100000".to_string(), |v| v).parse::<u32>().map_err(|e| ConfigError::ParseError("KEY_DERIVATION_ITERATIONS".to_string(), e.to_string()))?,
            },
            ukey: UKeyConfig {
                vendor: get_env_var("UKEY_VENDOR").map_or("default".to_string(), |v| v),
                api_url: get_env_var("UKEY_API_URL").map_or("http://localhost:8080/ukey".to_string(), |v| v),
            },
            tmdb: TmdbConfig {
                api_key: get_env_var("TMDB_API_KEY").map_or("".to_string(), |v| v),
                api_url: get_env_var("TMDB_API_URL").map_or("https://api.themoviedb.org/3".to_string(), |v| v),
                enabled: get_env_var("TMDB_ENABLED").map_or("false".to_string(), |v| v).parse::<bool>().map_err(|e| ConfigError::ParseError("TMDB_ENABLED".to_string(), e.to_string()))?,
            },
            image: ImageConfig {
                compression_quality: get_env_var("IMAGE_COMPRESSION_QUALITY").map_or("80".to_string(), |v| v).parse::<u8>().map_err(|e| ConfigError::ParseError("IMAGE_COMPRESSION_QUALITY".to_string(), e.to_string()))?,
                max_width: get_env_var("IMAGE_MAX_WIDTH").map_or("1920".to_string(), |v| v).parse::<u32>().map_err(|e| ConfigError::ParseError("IMAGE_MAX_WIDTH".to_string(), e.to_string()))?,
                max_height: get_env_var("IMAGE_MAX_HEIGHT").map_or("1080".to_string(), |v| v).parse::<u32>().map_err(|e| ConfigError::ParseError("IMAGE_MAX_HEIGHT".to_string(), e.to_string()))?,
            },
            log: LogConfig {
                level: get_env_var("LOG_LEVEL").map_or("info".to_string(), |v| v),
                file: get_env_var("LOG_FILE").map_or("./logs/gcm.log".to_string(), |v| v),
            },
            cors: CorsConfig {
                allow_origins: get_env_var("CORS_ALLOW_ORIGINS").map_or("*".to_string(), |v| v),
                allow_methods: get_env_var("CORS_ALLOW_METHODS").map_or("GET,POST,PUT,DELETE,OPTIONS".to_string(), |v| v),
                allow_headers: get_env_var("CORS_ALLOW_HEADERS").map_or("*".to_string(), |v| v),
            },
            upload: UploadConfig {
                max_size: get_env_var("UPLOAD_MAX_SIZE").map_or("104857600".to_string(), |v| v).parse::<u64>().map_err(|e| ConfigError::ParseError("UPLOAD_MAX_SIZE".to_string(), e.to_string()))?,
                temp_dir: get_env_var("UPLOAD_TEMP_DIR").map_or("./uploads".to_string(), |v| v),
            },
        })
    }
    
    /// 检查配置是否有效
    pub fn validate(&self) -> Result<(), ConfigError> {
        // 验证必要的配置项
        if self.jwt.secret.is_empty() {
            return Err(ConfigError::MissingEnvVar("JWT_SECRET".to_string()));
        }
        
        if self.encryption.salt.is_empty() {
            return Err(ConfigError::MissingEnvVar("ENCRYPTION_SALT".to_string()));
        }
        
        // 验证数据库URL
        if !self.database.url.starts_with("sqlite:") && !self.database.url.starts_with("postgresql:") {
            return Err(ConfigError::ParseError("DATABASE_URL".to_string(), "只支持SQLite或PostgreSQL数据库".to_string()));
        }
        
        Ok(())
    }
    
    /// 获取数据库路径（如果是SQLite）
    #[allow(dead_code)]
    pub fn get_database_path(&self) -> Option<&Path> {
          if let Some(path) = self.database.url.strip_prefix("sqlite:") {
              Some(Path::new(path))
          } else {
              None
          }
      }
    
    /// 是否为开发环境
    #[allow(dead_code)]
    pub fn is_development(&self) -> bool {
        self.server.env == "development"
    }
    
    /// 是否为生产环境
    #[allow(dead_code)]
    pub fn is_production(&self) -> bool {
        self.server.env == "production"
    }
}

/// 替换字符串中的环境变量占位符
fn replace_env_placeholders(value: &str) -> String {
    let mut result = value.to_string();
    
    // 替换 ${var} 格式的占位符
    let mut start = 0;
    while let Some(left) = result[start..].find("${") {
        let left_pos = start + left;
        if let Some(right) = result[left_pos..].find("}") {
            let right_pos = left_pos + right;
            let var_name = &result[left_pos + 2..right_pos];
            
            // 获取环境变量值
            if let Some(var_value) = env::var(var_name).ok() {
                // 替换占位符
                result.replace_range(left_pos..right_pos + 1, &var_value);
                // 更新start位置
                start = left_pos + var_value.len();
            } else {
                // 如果环境变量不存在，保留占位符
                start = right_pos + 1;
            }
        } else {
            break;
        }
    }
    
    result
}

/// 获取环境变量，如果不存在则返回默认值，并替换占位符
fn get_env_var(key: &str) -> Option<String> {
    env::var(key).ok().map(|v| replace_env_placeholders(&v))
}

lazy_static::lazy_static! {
    /// 全局配置实例
    pub static ref CONFIG: AppConfig = {
        AppConfig::from_env().unwrap_or_else(|e| {
            eprintln!("配置加载失败: {}", e);
            std::process::exit(1);
        })
    };
}
