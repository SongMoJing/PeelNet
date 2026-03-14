use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::layout::Rect;
use ratatui::Frame;

pub trait Component {
	/// 获取元素的位置
	fn position(&self) -> (i32, i32);
	/// 获取元素大小
	fn size(&self) -> (u16, u16);
	/// 绘制元素
	fn draw(&self, area: Rect, f: &mut Frame);
	/// 处理事件
	fn handle_event(&mut self, event: ComponentEvent) -> bool;
}

pub enum ComponentEvent {
	Key(KeyEvent),
	Mouse(MouseEvent),
}
