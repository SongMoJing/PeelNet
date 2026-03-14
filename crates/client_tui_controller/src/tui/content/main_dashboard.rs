use crate::tui::component::component_empty::EmptyComponent;
use crate::tui::component::component_nav::{MenuItem, NavComponent};
use crate::tui::component::{Component, ComponentEvent};
use crate::tui::content::Context;
use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::Frame;
use std::rc::Rc;

pub struct Dashboard {
    context: Rc<Box<dyn Context>>,
    nav: Box<dyn Component>,
}

impl Component for Dashboard {
    fn new(context: Rc<Box<dyn Context>>) -> Self
    where
        Self: Sized,
    {
        let mut nav = NavComponent::new(context.clone());
        nav.add_item(MenuItem::new(
            'C',
            "连接".to_string(),
            Box::new(EmptyComponent::new(context.clone())),
        ));
        nav.add_item(MenuItem::new(
            'N',
            "网络".to_string(),
            Box::new(EmptyComponent::new(context.clone())),
        ));
        nav.add_item(MenuItem::new(
            'S',
            "设置".to_string(),
            Box::new(EmptyComponent::new(context.clone())),
        ));
        Dashboard {
            nav: Box::new(nav),
            context,
        }
    }

    fn children(&mut self) -> Vec<&Box<dyn Component>> {
        vec![&self.nav]
    }

    fn draw(&mut self, area: Rect, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(0)])
            .split(area);
        self.nav.draw(chunks[0], f);
    }

    fn handle_event(&mut self, event: ComponentEvent) -> bool {
        if self.nav.handle_event(event.clone()) {
            return true;
        }
        match event {
            ComponentEvent::Key(key) => {
                if key.code == KeyCode::Char('q') && key.modifiers == KeyModifiers::CONTROL {
                    self.get_window().exit(0)
                }
            }
            _ => {}
        }

        false
    }

    fn get_window(&mut self) -> Rc<Box<dyn Context>> {
        self.context.clone()
    }
}
