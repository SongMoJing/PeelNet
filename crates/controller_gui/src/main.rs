use iced::widget::{button, column, text, Column};
use iced::window::{icon, Icon, Settings};

fn main() -> iced::Result {
    iced::application(Counter::default, Counter::update, Counter::view)
        .window(Settings {
            icon: Some(Icon::from(icon::from_file_data(include_bytes!("../res/icon.ico"), None).unwrap())),
            ..Default::default()
        })
        .run()
}

#[derive(Default)]
struct Counter {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement,
}

impl Counter {
    pub fn view(&self) -> Column<'_, Message> {
        column![
            button("添加").on_press(Message::Increment),
            text(&self.value).size(50),
            button("减少").on_press(Message::Decrement),
        ]
    }
}

impl Counter {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }
}