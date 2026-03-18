#![allow(unused_variables)]

use chrono::{DateTime, Local};
use colored::Colorize;
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
		let time = format!("[{}]", self.time.format("%H:%M:%S%.3f"));
		let flag = match self.level {
			LogLevel::Debug => " <Debug> ".on_bright_cyan().bright_white(),
			LogLevel::Info => " <Info>  ".on_bright_blue().bright_white(),
			LogLevel::Warn => " <Warn>  ".on_bright_yellow().bright_white(),
			LogLevel::Error => " <Error> ".on_bright_red().bright_white(),
		};
		let tag = self.tag.bright_white();
		let min_len = 24;
		(time.len() + max(tag.len(), min_len) + flag.len() + 3, format!(
			"{} {:>min_len$} {} ",
			time,
			tag,
			flag,
		))
	}
}
