mod cell;
pub use cell::*;
mod color;
pub use color::*;
mod event;
pub use event::*;
mod terminal;
pub use terminal::*;
mod canvas;
pub use canvas::*;
pub mod geom;

use std::io;
use std::io::Write;
use std::sync::mpsc::{Receiver, TryRecvError};

pub struct TerminalCanvas {
	term: Terminal,
	front_buffer: Canvas,
	back_buffer: Canvas,
	event_rx: Receiver<Event>,
}

impl TerminalCanvas {
	pub fn new() -> Self {
		let mut term = Terminal::new();
		let event_rx = term.event_channel();
		let front_buffer = Canvas::new(term.width(), term.height());
		let back_buffer = Canvas::new(term.width(), term.height());
		Self {
			term,
			front_buffer,
			back_buffer,
			event_rx,
		}
	}

	pub fn canvas(&self) -> &Canvas {
		&self.back_buffer
	}

	pub fn canvas_mut(&mut self) -> &mut Canvas {
		&mut self.back_buffer
	}

	pub fn width(&self) -> u32 {
		self.term.width()
	}

	pub fn height(&self) -> u32 {
		self.term.height()
	}

	pub fn next_event(&mut self) -> Result<Event, TryRecvError> {
		self.event_rx.try_recv()
	}

	pub fn attach(&mut self) -> Result<(), io::Error> {
		let term = &mut self.term;
		term.listen_for_resize();
		term.alt_screen()?;
		term.hide_cursor()?;
		term.raw_mode();
		term.enable_mouse()?;
		term.enable_mouse_move()?;
		term.flush()?;
		term.listen_for_events();
		term.clear()?;
		term.move_to(0, 0)?;

		Ok(())
	}

	pub fn detach(&mut self) -> Result<(), io::Error> {
		let term = &mut self.term;
		term.main_screen()?;
		term.show_cursor()?;
		term.normal_mode();
		term.disable_mouse_move()?;
		term.disable_mouse()?;
		term.flush()?;

		Ok(())
	}

	pub fn set_cell(&mut self, x: i32, y: i32, cell: Cell) {
		if let Some(dst) = self.canvas_mut().cell_mut(x, y) {
			*dst = cell;
		}
	}

	pub fn clear(&mut self) {
		self.canvas_mut().clear();
	}

	pub fn refresh_size(&mut self) {
		let (w, h) = self.term.size();
		if w != self.back_buffer.width() || h != self.back_buffer.height() {
			self.back_buffer.resize(w, h);
			self.front_buffer.resize(w, h);
		}
	}

	pub fn present(&mut self) -> Result<(), io::Error> {
		self.refresh_size();
		let front = &mut self.front_buffer;
		let back = &mut self.back_buffer;
		let term = &mut self.term;

		let (w, h) = front.size();
		let (mut cursor_x, mut cursor_y) = (-1, -1);
		let (mut cursor_fg, mut cursor_bg) = (Color::transparent(), Color::transparent());
		term.reset()?;
		for y in 0..h as i32 {
			for x in 0..w as i32 {
				if let Some(back_cell) = back.cell(x, y) {
					if let Some(front_cell) = front.cell_mut(x, y) {
						if back_cell == front_cell {
							continue;
						}

						if x != cursor_x || y != cursor_y {
							cursor_x = x;
							cursor_y = y;
							term.move_to(x as u32, y as u32)?;
						}

						*front_cell = back_cell.clone();

						if front_cell.symbol == '\0' {
							if cursor_fg != Color::transparent() || cursor_bg != Color::transparent() {
								term.reset()?;
								cursor_fg = Color::transparent();
								cursor_bg = Color::transparent();
							}
							term.write_char(' ')?;
						} else {
							let fg = front_cell.fg.clone();
							let bg = {
								if front_cell.bg.a > 0 {
									front_cell.bg.clone()
								} else {
									Color::transparent()
								}
							};

							if cursor_bg != bg {
								cursor_bg = bg;
								if bg == Color::transparent() {
									term.reset()?;
									cursor_fg = Color::transparent();
								} else {
									term.bg_color(front_cell.bg)?;
								}
							}

							if cursor_fg != fg {
								cursor_fg = fg;
								term.fg_color(front_cell.fg)?;
							}

							term.write_char(front_cell.symbol)?;
						}
						// FIXME what about double width chars?
						cursor_x += 1;
					}
				}
			}
		}
		// Move cursor to the top so things don't jump around when resizing the terminal
		term.move_to(0, 0)?;
		term.flush()?;

		Ok(())
	}
}
