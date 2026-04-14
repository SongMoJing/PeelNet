use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum NetMessage {
	/**
	 * Ping 包
	 */
	Ping,
	/**
	 * 同步包
	 */
	Synchronization {
		/**
		 * 实时消息：CPU占用
		 */
		data_cpu: u8,
		/**
		 * 实时消息：网络上行带宽
		 */
		data_net_upload: usize,
		/**
		 * 实时消息：网络下行带宽
		 */
		data_net_download: usize,
		/**
		 * 实时消息：内存占用
		 */
		data_mem: usize,
	},
	/**
	 * 消息
	 */
	Message {
		id: u64,
		message: String,
	},
	/**
	 * 错误包
	 */
	Error {
		code: ErrorCode,
		message: String,
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ErrorCode {
}
