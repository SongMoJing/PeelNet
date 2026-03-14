use crossterm::event::KeyEvent;
use ratatui::Frame;
use ratatui::layout::Rect;

pub mod auth_screen;
pub mod main_dashboard;

pub trait Screen {
	fn render(&'_ self, area: Rect, f: &mut Frame);
	
	fn input(&mut self, key_event: KeyEvent);
}