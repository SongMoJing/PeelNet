use crate::tui::component::{Component, ComponentEvent};
use crate::tui::content::Context;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::Frame;
use std::rc::Rc;
use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::prelude::Style;
use ratatui::style::Color;
use ratatui::widgets::{Clear, Paragraph};

pub struct NavComponent {
    active_menu: isize,
    menu_group: Vec<Option<MenuItem>>,
    nav_rects: Vec<(usize, Rect)>,
    screen: Rc<Box<dyn Context>>,
}

impl Component for NavComponent {
    fn new(screen: Rc<Box<dyn Context>>) -> Self {
        NavComponent {
            screen,
            menu_group: vec![],
            active_menu: -1,
            nav_rects: vec![],
        }
    }

    fn children(&mut self) -> Vec<&Box<dyn Component>> {
        vec![]
    }

    fn draw(&mut self, area: Rect, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(self.menu_group.iter().map(|_| Constraint::Length(15)).collect::<Vec<_>>())
            .split(area);
        self.nav_rects.clear();
        for (index, item) in self.menu_group.iter().enumerate() {
            if let Some(item) = item {
                let style = if self.active_menu == index as isize {
                    Style::default().bg(Color::Yellow).fg(Color::Black)
                } else {
                    Style::default().fg(Color::Yellow)
                };
                let nav_text = Paragraph::new(format!("[{}] {}", item.key, item.name)).style(style);
                f.render_widget(nav_text, chunks[index]);
                self.nav_rects.push((index, chunks[index]));
            }
        }
        if self.active_menu >= 0 {
            if let Some(Some(item)) = self.menu_group.get_mut(self.active_menu as usize) {
                let content_area = Rect::new(0, 2, area.width, 5);
                f.render_widget(Clear, content_area);
                item.content.draw(content_area, f);
            }
        }
    }

    fn handle_event(&mut self, event: ComponentEvent) -> bool {
        match event {
            ComponentEvent::Key(key) => {
                if key.code == KeyCode::Tab {
                    self.next_menu();
                    return true
                } else if key.code == KeyCode::Esc {
                    self.close_menu();
                    return true
                } else if key.modifiers == KeyModifiers::ALT {
                    for (index, item) in self.menu_group.iter().enumerate() {
                        if let Some(item) = item {
                            if let KeyCode::Char(c) = key.code {
                                if item.key.to_lowercase().to_string() == c.to_lowercase().to_string() {
                                    self.active_menu = index as isize;
                                    return true
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        false
    }

    fn get_window(&mut self) -> Rc<Box<dyn Context>> {
        self.screen.clone()
    }
}

impl NavComponent {
    pub fn close_menu(&mut self) {
        self.active_menu = -1;
    }

    pub fn change_to_menu(&mut self, char: char) {
        for (index, item) in self.menu_group.iter().enumerate() {
            if let Some(item) = item {
                if item.key.to_lowercase().to_string() == char.to_lowercase().to_string() {
                    self.active_menu = index as isize;
                    return;
                }
            }
        }
    }

    pub fn next_menu(&mut self) {
        if self.active_menu < 0 {
            self.active_menu = 1;
            return
        }
        self.active_menu = ((self.active_menu as usize + 1) % self.menu_group.len()) as isize;
    }

    pub fn add_item(&mut self, item: MenuItem) {
        self.menu_group.push(Some(item));
    }

    pub fn insert_item(&mut self, index: usize, item: MenuItem) {
        self.menu_group.insert(index, Some(item));
    }

    pub fn remove_item(&mut self, index: usize) {
        self.menu_group.remove(index);
    }
}

pub struct MenuItem {
    key: char,
    name: String,
    content: Box<dyn Component>,
}

impl MenuItem {
    pub fn new(key: char, name: String, content: Box<dyn Component>) -> Self {
        MenuItem {
            key,
            name,
            content,
        }
    }
}
