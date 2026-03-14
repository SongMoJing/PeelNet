use cursive::CursiveRunnable;

pub mod index;

pub trait Page {
	fn render(&mut self, siv: &mut CursiveRunnable);
}
