use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;
use crate::tui::screen::Screen;

pub struct AuthScreen {
    pub(crate) password: String,
    pub(crate) status_message: String,
}

impl AuthScreen {
    pub fn new() -> Self {
        AuthScreen {
            password: String::new(),
            status_message: String::new(),
        }
    }
}

impl Screen for AuthScreen {
    fn render(&'_ self, area: Rect, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(1),
            ])
            .split(area);
        let title = Paragraph::new("PeeL-TUI 身份验证")
            .alignment(Alignment::Center)
            .block(Block::default());
        f.render_widget(title, chunks[0]);
        let masked_password: String = "*".repeat(self.password.chars().count());
        let input = Paragraph::new(masked_password).block(
            Block::default()
                .borders(Borders::ALL)
                .title("请输入证书提取密码 (Enter 确认)"),
        );
        f.render_widget(input, chunks[1]);
        let help = Paragraph::new(Text::raw(&self.status_message))
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center);
        f.render_widget(help, chunks[2]);
    }

    fn input(&mut self, key_event: KeyEvent) {
        match key_event {
            KeyEvent { code, .. } => match code {
                KeyCode::Backspace => {
                    self.password.pop();
                }
                KeyCode::Enter => {
                    self.status_message = "正在验证...".to_string();
                }
                KeyCode::Char(c) => {
                    self.password.push(c);
                }
                _ => return,
            },
        }
    }
}
