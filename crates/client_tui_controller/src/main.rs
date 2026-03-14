mod tui;

use crate::tui::screen::main_dashboard::{Dashboard};
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, KeyEvent, KeyModifiers};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::{CrosstermBackend, Terminal};
use std::io::{stdout, Result};
use std::time::Duration;
use crate::tui::screen::auth_screen::AuthScreen;

fn main() -> Result<()> {
    let mut app = Dashboard::new(Box::new(AuthScreen::new()));
    stdout().execute(EnterAlternateScreen)?;
    stdout().execute(EnableMouseCapture)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    loop {
        terminal.draw(|f| {
            app.draw(f);
        })?;
        if event::poll(Duration::from_millis(100))? {
            let event = event::read()?;
            if let Event::Key(key_event) = event {
                match key_event {
                    // Ctrl Q -> 退出
                    KeyEvent {
                        code: KeyCode::Char('q'),
                        modifiers: KeyModifiers::CONTROL,
                        ..
                    } => {
                        break;
                    }
                    // Tab -> 菜单
                    KeyEvent {
                        code: KeyCode::Tab, ..
                    } => app.next_menu(),
                    // Alt Char -> 菜单
                    KeyEvent {
                        code: c,
                        modifiers: KeyModifiers::ALT,
                        ..
                    } => {
                        if let Some(c) = c.as_char() {
                            app.change_to_menu(c)
                        }
                    }
                    KeyEvent {
                        code: KeyCode::Esc, ..
                    } => app.close_menu(),
                    _ => {
                        app.handle_key_event(key_event);
                    }
                }
            }
            if let Event::Mouse(mouse_event) = event {
                app.handle_mouse_event(mouse_event);
            }
        }
    }
    // 终端退出
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    stdout().execute(DisableMouseCapture)?;
    Ok(())
}
