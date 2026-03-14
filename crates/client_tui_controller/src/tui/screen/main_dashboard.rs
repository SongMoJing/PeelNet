use crossterm::event;
use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::layout::{Constraint, Direction, Layout, Position, Rect};
use ratatui::prelude::{Color, Style};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui::Frame;
use ratatui::style::Stylize;
use crate::tui::screen::Screen;

pub struct Dashboard {
    active_menu: usize,
    menu_group: Vec<Option<MenuItem>>,
    nav_rects: Vec<(usize, Rect)>,
    screen: Box<dyn Screen>,
}

impl Dashboard {
    pub fn new(screen: Box<dyn Screen>) -> Self {
        Dashboard {
            active_menu: 0,
            menu_group: vec![
                None,
                Some(MenuItem {
                    key: 'C',
                    name: "连接".to_string(),
                    content: Box::new(String::from("连接")),
                }),
                Some(MenuItem {
                    key: 'N',
                    name: "网络".to_string(),
                    content: Box::new(String::from("网络")),
                }),
                Some(MenuItem {
                    key: 'S',
                    name: "设置".to_string(),
                    content: Box::new(String::from("设置")),
                }),
            ],
            nav_rects: vec![],
            screen
        }
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        self.screen.input(key_event);
    }

    pub fn handle_mouse_event(&mut self, mouse_event: MouseEvent) {
        let pos = (mouse_event.column, mouse_event.row);
        if mouse_event.kind == event::MouseEventKind::Down(event::MouseButton::Left) {
            let mouse_pos = Position::new(pos.0, pos.1);
            for (i, rect) in self.nav_rects.iter() {
                if rect.contains(mouse_pos) {
                    if *i == self.active_menu {
                        self.close_menu();
                    } else {
                        self.active_menu = *i;
                    }
                    break;
                }
            }
        }
    }
}

impl Dashboard {
    pub fn draw(&mut self, f: &mut Frame) {
        let canvas_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(0)])
            .split(f.area());
        self.screen.render(canvas_chunks[1], f);
        let nav_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(20), Constraint::Min(0)])
            .split(canvas_chunks[0]);
        let canvas_nav = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Min(10),
                Constraint::Min(10),
                Constraint::Min(10),
                Constraint::Percentage(100),
            ])
            .split(nav_layout[0]);
        self.nav_rects.clear();
        for (index, item) in self.menu_group.iter().enumerate() {
            if let Some(item) = item {
                let mut nav = Paragraph::new(format!("[{}] {}", item.key, item.name))
                    .style(Style::default().fg(Color::Yellow));
                if self.active_menu == index {
                    nav = nav.style(Style::default().bg(Color::Yellow).fg(Color::White));
                }
                f.render_widget(nav, canvas_nav[index - 1]);
                self.nav_rects.push((index, canvas_nav[index - 1]));
            }
        }
        if let Some(active) = &self.menu_group[self.active_menu] {
            let x = canvas_nav[0].width * (self.active_menu as u16 - 1);
            let popup_area = Rect::new(x, 1, 10, 4);
            let content = active.content.render()
                .bg(Color::DarkGray)
                .fg(Color::White)
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(Clear, popup_area);
            f.render_widget(content, popup_area);
        }
    }

    pub fn close_menu(&mut self) {
        self.active_menu = 0;
    }

    pub fn change_to_menu(&mut self, char: char) {
        for (index, item) in self.menu_group.iter().enumerate() {
            if let Some(item) = item {
                if item.key.to_lowercase().to_string() == char.to_lowercase().to_string() {
                    self.active_menu = index;
                    return;
                }
            }
        }
    }

    pub fn next_menu(&mut self) {
        self.active_menu = (self.active_menu + 1) % self.menu_group.len();
    }
}

struct MenuItem {
    key: char,
    name: String,
    content: Box<dyn Renderable>,
}

pub trait Renderable {
    fn render(&'_ self) -> Paragraph<'_>;
}

impl Renderable for String {
    fn render(&self) -> Paragraph<'_> {
        Paragraph::new(self.clone())
    }
}
