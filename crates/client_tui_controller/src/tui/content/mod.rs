use std::cell::RefCell;

pub mod auth_screen;
pub mod main_dashboard;

pub trait Context {
	/// 退出程序并返回终端命令行
	fn exit(&self, code: i32);
}

pub struct ImplContext {
	exit_callback: RefCell<Box<dyn Fn(i32)>>,
}

impl ImplContext {
	pub fn new(exit: Box<dyn Fn(i32)>) -> Self {
		Self {
			exit_callback: RefCell::new(exit),
		}
	}
}

impl Context for ImplContext {
	fn exit(&self, code: i32) {
		(self.exit_callback.borrow_mut())(code);
	}
}
