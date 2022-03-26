use crate::{Color, Event};
use std::io;
use std::sync::{mpsc, Arc, Mutex};

mod events;
mod flags;
mod input;
mod modes;
mod resize;

pub type WriteResult = Result<usize, io::Error>;

pub struct Terminal {
	event_channels: Arc<Mutex<Vec<mpsc::Sender<Event>>>>,
	width: u32,
	height: u32,
	font_width: u32,
	font_height: u32,
}

impl Terminal {
	pub fn new() -> Self {
		let listeners: Arc<Mutex<Vec<mpsc::Sender<Event>>>> = Arc::new(Mutex::new(vec![]));
		Self {
			event_channels: listeners.clone(),
			width: 120,
			height: 48,
			font_width: 16,
			font_height: 32,
		}
	}

	pub fn size(&self) -> (u32, u32) {
		(self.width(), self.height())
	}

	pub fn flush(&mut self) -> Result<(), io::Error> {
		Ok(())
	}

	pub fn write(&mut self, _output: &str) -> WriteResult {
		Ok(0)
	}

	pub fn write_char(&mut self, _output: char) -> WriteResult {
		Ok(0)
	}

	pub fn csi(&mut self, output: &str) -> WriteResult {
		self.write(&format!("\x1b[{}", output))
	}

	pub fn clear(&mut self) -> WriteResult {
		self.reset()?;
		self.csi("2J")
	}

	pub fn alt_screen(&mut self) -> WriteResult {
		self.csi("?1049h")
	}

	pub fn main_screen(&mut self) -> WriteResult {
		self.csi("?1049l")
	}

	pub fn move_to(&mut self, x: u32, y: u32) -> WriteResult {
		self.csi(&format!("{};{}H", y + 1, x + 1))
	}

	pub fn show_cursor(&mut self) -> WriteResult {
		self.csi("?25h")
	}

	pub fn hide_cursor(&mut self) -> WriteResult {
		self.csi("?25l")
	}

	pub fn fg_color(&mut self, color: Color) -> WriteResult {
		let (r, g, b) = color.as_rgb();
		self.csi(&format!("38;2;{};{};{}m", r, g, b))
	}

	pub fn bg_color(&mut self, color: Color) -> WriteResult {
		let (r, g, b) = color.as_rgb();
		self.csi(&format!("48;2;{};{};{}m", r, g, b))
	}

	pub fn fg_8bit_color(&mut self, color: Color) -> WriteResult {
		let code = color.as_8bit_ansi();
		self.csi(&format!("38;5;{}m", code))
	}

	pub fn bg_8bit_color(&mut self, color: Color) -> WriteResult {
		let code = color.as_8bit_ansi();
		self.csi(&format!("48;5;{}m", code))
	}

	pub fn color(&mut self, fg: Color, bg: Color) -> WriteResult {
		Ok(self.fg_color(fg)? + self.bg_color(bg)?)
	}

	pub fn reset(&mut self) -> WriteResult {
		self.csi("m")
	}
}
