pub mod component_empty;
pub mod component_nav;

use std::rc::Rc;
use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::layout::Rect;
use ratatui::Frame;
use crate::tui::content::Context;

pub trait Component {
	/// 构造
	fn new(context: Rc<Box<dyn Context>>) -> Self where Self: Sized;
	/// 获取子元素
	fn children(&mut self) -> Vec<&Box<dyn Component>>;
	/// 绘制元素
	fn draw(&mut self, area: Rect, f: &mut Frame);
	/// 处理事件
	fn handle_event(&mut self, event: ComponentEvent) -> bool;

	fn get_window(&mut self) -> Rc<Box<dyn Context>>;
}

#[derive(Debug, Clone, Copy)]
pub enum ComponentEvent {
	Key(KeyEvent),
	Mouse(MouseEvent),
}
