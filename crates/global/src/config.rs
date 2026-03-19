use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
	/// 服务器配置 面向管理员
	pub controller: ControllerConfig,
	/// 数据源配置
	pub data_sources: DataSourcesConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ControllerConfig {
	/// WebUI配置
	pub web_ui: Option<WebUIConfig>,
	/// 网络控制器配置
	pub network_controller: Option<NetworkControllerConfig>,
	/// CA配置
	pub ca: CAConfig,
	/// JWT配置
	pub jwt: JWTConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WebUIConfig {
	/// 服务器监听地址
	pub addr: String,
	/// 服务监听端口
	pub port: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NetworkControllerConfig {
	/// 服务器监听地址
	pub addr: String,
	/// 服务监听端口
	pub port: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CAConfig {
	/// 根证书
	pub cert_ca: String,
	/// 证书
	pub cert_server: String,
	/// 密钥
	pub cert_key: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JWTConfig {
	/// 过期时间（秒）
	pub expiration: u64,
	/// 密钥
	pub secret: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DataSourcesConfig {
	pub redis: RedisConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RedisConfig {
	/// 数据库URL
	pub url: String,
	/// 连接超时
	pub timeout_sec: u64,
	/// 最大连接数
	pub max_connections: u32,
	/// 最大空闲连接数
	pub max_idle_connections: u32,
}
