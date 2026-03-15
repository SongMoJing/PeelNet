use std::rc::Rc;
use cursive::views::LinearLayout;
use crate::model::config::Config;
use crate::model::saved::Saved;
use crate::model::state::State;

pub mod index;
pub mod menu;

pub trait Page {
	fn render(&mut self, config: Rc<Config>, state: Rc<State>, saved: Rc<Saved>) -> LinearLayout;
}
