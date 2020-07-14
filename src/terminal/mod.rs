use crate::{Color, Event};
use std::io::{self, Write};
use std::sync::{mpsc, Arc, Mutex};
use std::thread::JoinHandle;

mod events;
mod flags;
mod input;
mod modes;
mod resize;

pub type WriteResult = Result<usize, io::Error>;

pub struct Terminal {
	event_channels: Arc<Mutex<Vec<mpsc::Sender<Event>>>>,
	event_thread: Option<JoinHandle<()>>,
	stdout: io::Stdout,
	stdin: io::Stdin,
}

impl Terminal {
	pub fn new() -> Self {
		let listeners = Arc::new(Mutex::new(vec![]));
		let term = Self {
			event_channels: listeners.clone(),
			event_thread: None,
			stdout: io::stdout(),
			stdin: io::stdin(),
		};

		term.refresh_size();

		term
	}

	pub fn stdin(&self) -> &io::Stdin {
		&self.stdin
	}

	pub fn stdout(&self) -> &io::Stdout {
		&self.stdout
	}

	pub fn stdin_mut(&mut self) -> &mut io::Stdin {
		&mut self.stdin
	}

	pub fn stdout_mut(&mut self) -> &mut io::Stdout {
		&mut self.stdout
	}

	pub fn size(&self) -> (u32, u32) {
		(self.width(), self.height())
	}

	pub fn flush(&mut self) -> Result<(), io::Error> {
		self.stdout.flush()
	}

	pub fn write(&mut self, output: &str) -> WriteResult {
		self.stdout.write(output.as_bytes())
	}

	pub fn write_char(&mut self, output: char) -> WriteResult {
		let mut bytes = [0; 4];
		output.encode_utf8(&mut bytes);
		self.stdout.write(&bytes)
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
