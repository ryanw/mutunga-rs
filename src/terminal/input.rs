use super::Terminal;
use crate::{Event, MouseButton};
use libc::{c_int, sighandler_t, signal, SIGINT};
use std::io::{self, Bytes, Read};
use std::iter::Peekable;
use std::thread;

const ESC: char = '\x1b';

extern "C" fn handle_interrupt(_sig: c_int) {
	// Hacks to cleanup the terminal
	let mut term = Terminal::new();
	term.main_screen();
	term.show_cursor();
	term.normal_mode();
	term.disable_mouse_move();
	term.disable_mouse();
	term.flush();
	std::process::exit(0);
}

impl Terminal {
	pub fn listen_for_events(&mut self) {
		// Catch ctrl+c and cleanup
		// FIXME do this in a nicer way
		unsafe {
			signal(SIGINT, handle_interrupt as sighandler_t);
		}

		let listeners = self.event_channels.clone();
		self.event_thread = Some(thread::spawn(move || {
			let mut parser = InputParser::new(io::stdin());
			while let Some(event) = parser.next_event() {
				for tx in &*listeners.lock().unwrap() {
					tx.send(event.clone()).expect("Failed to send event");
				}
			}
		}));
	}
}

fn is_ctrl_char(c: char) -> bool {
	match c {
		'D' | 'E' | 'H' | 'M' | 'N' | 'O' | 'P' | 'V' | 'W' | 'X' | 'Z' | '[' | '\\' | ']' | '^' | '_' => true,
		_ => false,
	}
}

pub struct InputParser<R: Read> {
	stream: Peekable<Bytes<R>>,
}

impl<R: Read> InputParser<R> {
	pub fn new(stream: R) -> Self {
		Self {
			stream: stream.bytes().peekable(),
		}
	}

	fn read_esc_sequence(&mut self) -> Event {
		match self.next_char() {
			'[' => self.read_ctrl_sequence(),
			// TODO other sequence types
			_ => Event::Unknown,
		}
	}

	fn read_ctrl_sequence(&mut self) -> Event {
		match self.next_char() {
			'M' => self.read_x10_mouse(),
			'<' => self.read_sgr_mouse(),
			c => Event::KeyPress(c),
		}
	}

	fn read_x10_mouse(&mut self) -> Event {
		// TODO
		Event::Unknown
	}

	fn read_sgr_mouse(&mut self) -> Event {
		let mut mouse_move = false;
		let mut mouse_up = false;
		// Read mouse button
		let button = match self.next_char() {
			'0' => MouseButton::Left,
			'1' => MouseButton::Middle,
			'2' => MouseButton::Right,
			// Moving
			'3' => {
				mouse_move = true;
				match self.next_char() {
					'2' => MouseButton::Left,
					'3' => MouseButton::Middle,
					'4' => MouseButton::Right,
					'5' => MouseButton::None,
					_ => MouseButton::Unknown,
				}
			}
			// Scrolling
			'6' => match self.next_char() {
				'4' => MouseButton::WheelUp,
				'5' => MouseButton::WheelDown,
				_ => MouseButton::Unknown,
			},
			_ => MouseButton::Unknown,
		};

		// Next byte should be a ';'
		if self.next_char() != ';' {
			// Invalid mouse coords
			return Event::Unknown;
		}

		let mut x = 0;
		let mut y = 0;
		let mut token = 0;
		let mut ascii_num = String::new();
		self.scan_chars(|c| {
			if c == ';' || c == 'm' || c == 'M' {
				if token == 0 {
					x = ascii_num.parse::<u32>().unwrap_or(1) - 1;
					ascii_num = String::new();
				} else if token == 1 {
					y = ascii_num.parse::<u32>().unwrap_or(1) - 1;
					ascii_num = String::new();
				} else {
					// 3D mouse?!
					// Too many mouse coords
					return false;
				}
				token += 1;

				if c == 'm' {
					mouse_up = true;
				}

				// scan until we hit an 'm' or 'M'
				return c != 'm' && c != 'M';
			}

			// Read coord digit
			if c >= '0' && c <= '9' {
				ascii_num.push(c);
				true
			} else {
				// Invalid mouse position
				false
			}
		});

		if mouse_move {
			Event::MouseMove(button, x, y)
		} else if mouse_up {
			Event::MouseUp(button, x, y)
		} else {
			Event::MouseDown(button, x, y)
		}
	}

	fn scan_chars<F: FnMut(char) -> bool>(&mut self, mut scanner: F) {
		while scanner(self.next_char()) {}
	}

	pub fn next_event(&mut self) -> Option<Event> {
		let c = self.next_char();

		if c == ESC && is_ctrl_char(self.peek_char()) {
			// Parsing escape sequence
			Some(self.read_esc_sequence())
		} else {
			// Normal key press
			Some(Event::KeyPress(c))
		}
	}

	pub fn next_byte(&mut self) -> u8 {
		if let Some(Ok(byte)) = self.stream.next() {
			byte
		} else {
			// Failed to read byte
			0
		}
	}

	pub fn next_char(&mut self) -> char {
		self.next_byte() as _
	}

	pub fn peek_byte(&mut self) -> u8 {
		if let Some(Ok(byte)) = self.stream.peek() {
			*byte
		} else {
			// Failed to peek byte
			0
		}
	}

	pub fn peek_char(&mut self) -> char {
		self.peek_byte() as _
	}
}
