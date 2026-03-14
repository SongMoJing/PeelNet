use crate::tui::component::Component;

struct UITree {
	root: Box<dyn Component>,
}
