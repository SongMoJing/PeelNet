#![allow(unused_variables)]

use chrono::{DateTime, Local};
use colored::Colorize;
use unicode_width::UnicodeWidthStr;
use std::cmp::max;

pub struct Log {
	time: DateTime<Local>,
	level: LogLevel,
	tag: String,
	message: String,
}

pub enum LogLevel {
	Debug,
	Info,
	Warn,
	Error,
}

impl Log {
	fn new(level: LogLevel, tag: String, message: String) -> Self {
		Log {
			time: Local::now(),
			level,
			tag,
			message,
		}
	}

	pub fn d<T: Into<String>, M: Into<String>>(tag: T, msg: M) -> Self { Self::new(LogLevel::Debug, tag.into(), msg.into()) }
	pub fn i<T: Into<String>, M: Into<String>>(tag: T, msg: M) -> Self { Self::new(LogLevel::Info, tag.into(), msg.into()) }
	pub fn w<T: Into<String>, M: Into<String>>(tag: T, msg: M) -> Self { Self::new(LogLevel::Warn, tag.into(), msg.into()) }
	pub fn e<T: Into<String>, M: Into<String>>(tag: T, msg: M) -> Self { Self::new(LogLevel::Error, tag.into(), msg.into()) }

	pub fn to_string(&self) -> String {
		let (head_width, colored_head) = self.create_head();
		let indent = " ".repeat(head_width);
		self.message.lines().enumerate().fold(String::new(), |mut acc, (i, line)| {
			if i == 0 {
				acc += &format!("{}{}\n", colored_head, line);
			} else {
				acc += &format!("{}{}\n", indent, line);
			}
			acc
		})
	}

	pub fn print(&self) {
		print!("{}", self.to_string())
	}

	fn create_head(&self) -> (usize, String) {
		let time_str = self.time.format("%H:%M:%S%.3f").to_string();
		let time_display = format!("[{}]", time_str);
		let time_width = time_display.width();
		let flag_text = match self.level {
			LogLevel::Debug => " <Debug> ",
			LogLevel::Info  => " <Info>  ",
			LogLevel::Warn  => " <Warn>  ",
			LogLevel::Error => " <Error> ",
		};
		let flag_colored = match self.level {
			LogLevel::Debug => flag_text.on_bright_cyan().bright_white(),
			LogLevel::Info  => flag_text.on_bright_blue().bright_white(),
			LogLevel::Warn  => flag_text.on_bright_yellow().bright_white(),
			LogLevel::Error => flag_text.on_bright_red().bright_white(),
		};
		let flag_width = flag_text.width();
		let min_tag_width = 24;
		let actual_tag_width = self.tag.as_str().width();
		let padding_size = if actual_tag_width < min_tag_width {
			min_tag_width - actual_tag_width
		} else {
			0
		};
		let padded_tag = format!("{}{}", " ".repeat(padding_size), self.tag.bright_white());
		let final_tag_width = max(actual_tag_width, min_tag_width);
		let total_width = time_width + 1 + final_tag_width + 1 + flag_width + 1;
		let head_string = format!(
			"{} {} {} ",
			time_display,
			padded_tag,
			flag_colored,
		);
		(total_width, head_string)
	}
}
