mod tui;

use std::cell::RefCell;
use crate::tui::component::{Component, ComponentEvent};
use crate::tui::content::{Context, ImplContext};
use crate::tui::ui_tree::UITree;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::{
    event::{self, Event},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::{CrosstermBackend, Terminal};
use std::io::{stdout, Result};
use std::rc::Rc;
use std::time::Duration;
use crate::tui::content::main_dashboard::Dashboard;

fn main() -> Result<()> {
    let running = Rc::new(RefCell::new(true));
    let exit_code = Rc::new(RefCell::new(0));
    let running_clone = running.clone();
    let exit_code_clone = exit_code.clone();
    let context: Rc<Box<dyn Context>> = Rc::new(Box::new(ImplContext::new(Box::new(move |code| {
        *running_clone.borrow_mut() = false;
        *exit_code_clone.borrow_mut() = code;
    }))));
    let dashboard = Box::new(Dashboard::new(context.clone()));
    let mut tree = UITree::from(context, dashboard);
    stdout().execute(EnterAlternateScreen)?;
    stdout().execute(EnableMouseCapture)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    while *running.borrow() {
        terminal.draw(|f| {
            tree.root().draw(f.area(), f);
        })?;
        if event::poll(Duration::from_millis(100))? {
            let event = event::read()?;
            if let Event::Key(key_event) = event {
                tree.root().handle_event(ComponentEvent::Key(key_event));
            }
            if let Event::Mouse(mouse_event) = event {
                tree.root().handle_event(ComponentEvent::Mouse(mouse_event));
            }
        }
    }
    // 终端退出
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    stdout().execute(DisableMouseCapture)?;
    Ok(())
}
