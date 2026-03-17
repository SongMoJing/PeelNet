pub struct Saved {
	connected: Vec<ConnectInfo>
}

impl Saved {
	pub fn new() -> Saved {
		Saved {
			connected: vec![]
		}
	}

	pub fn add_connect_info(&mut self, info: ConnectInfo) {
		self.connected.push(info);
	}

	pub fn get_connect_info(&self, name: &str) -> Option<ConnectInfo> {
		for info in self.connected.iter() {
			if info.name == name {
				return Some(info.clone());
			}
		}
		None
	}

	pub fn remove_connect_info(&mut self, name: &str) {
		for i in 0..self.connected.len() {
			if self.connected[i].name == name {
				self.connected.remove(i);
				return;
			}
		}
	}

	pub fn list_connect_info(&self) -> Vec<ConnectInfo> {
		self.connected.clone()
	}
}

#[derive(Clone)]
pub struct ConnectInfo {
	/// 连接名称（备注）
	pub name: String,
	/// 目标主机
	pub host: String,
	/// 端口
	pub port: u16,
	/// 用户名
	pub user: String,
	/// 证书
	pub cert: String,
	/// 证书密码（可选）
	pub key: Option<String>,
}
