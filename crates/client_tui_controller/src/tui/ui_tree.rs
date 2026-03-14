use crate::tui::component::component_empty::EmptyComponent;
use crate::tui::component::Component;
use crate::tui::content::Context;
use std::rc::Rc;

pub struct UITree {
    context: Rc<Box<dyn Context>>,
    root: Box<dyn Component>,
}

impl UITree {
    pub fn new(context: Box<dyn Context>) -> Self {
        let ctx = Rc::new(context);
        let root = Box::new(EmptyComponent::new(ctx.clone()));
        Self { context: ctx, root }
    }

    pub fn from(context: Rc<Box<dyn Context>>, root: Box<dyn Component>) -> Self {
        Self {
            context,
            root,
        }
    }

    pub fn root(&mut self) -> &mut Box<dyn Component> {
        &mut self.root
    }
}
