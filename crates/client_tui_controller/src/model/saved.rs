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
	pub name: String,
	pub host: String,
	pub port: u16,
	pub cert: String,
	pub key: String,
}