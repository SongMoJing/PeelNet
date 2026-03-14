use std::rc::Rc;
use ratatui::Frame;
use ratatui::layout::Rect;
use crate::tui::component::{Component, ComponentEvent};
use crate::tui::content::Context;

pub struct EmptyComponent {
	screen: Rc<Box<dyn Context>>
}

impl Component for EmptyComponent {
	fn new(screen: Rc<Box<dyn Context>>) -> Self {
		EmptyComponent {
			screen
		}
	}

	fn children(&mut self) -> Vec<&Box<dyn Component>> {
		vec![]
	}

	fn draw(&mut self, area: Rect, f: &mut Frame) {
	}

	fn handle_event(&mut self, event: ComponentEvent) -> bool {
		false
	}

	fn get_window(&mut self) -> Rc<Box<dyn Context>> {
		self.screen.clone()
	}
}