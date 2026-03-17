use cursive::View;
use cursive::views::LinearLayout;

pub mod index;
pub mod menu;

pub trait Page {
	fn build(&mut self) -> LinearLayout;
}
