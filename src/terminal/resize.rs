use crate::Event;
use libc::{c_int, ioctl, sighandler_t, signal, winsize, SIGWINCH, STDIN_FILENO, TIOCGWINSZ};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{mpsc, Mutex};
use std::thread;

use super::Terminal;

static TERM_WIDTH: AtomicU32 = AtomicU32::new(0);
static TERM_HEIGHT: AtomicU32 = AtomicU32::new(0);
static PIXEL_WIDTH: AtomicU32 = AtomicU32::new(0);
static PIXEL_HEIGHT: AtomicU32 = AtomicU32::new(0);
static FONT_WIDTH: AtomicU32 = AtomicU32::new(16);
static FONT_HEIGHT: AtomicU32 = AtomicU32::new(32);
static mut RESIZE_CHANNEL: Option<Mutex<mpsc::Sender<Event>>> = None;

extern "C" fn handle_resize(_sig: c_int) {
	let mut size = winsize {
		ws_row: 0,
		ws_col: 0,
		ws_xpixel: 0,
		ws_ypixel: 0,
	};
	unsafe {
		ioctl(STDIN_FILENO, TIOCGWINSZ, &mut size);
	}

	let term_width = size.ws_col as u32;
	let term_height = size.ws_row as u32;

	if TERM_WIDTH.load(Ordering::SeqCst) != term_width || TERM_HEIGHT.load(Ordering::SeqCst) != term_height {
		TERM_WIDTH.store(term_width, Ordering::SeqCst);
		TERM_HEIGHT.store(term_height, Ordering::SeqCst);
		unsafe {
			if let Some(chan) = RESIZE_CHANNEL.as_ref() {
				if let Ok(chan) = chan.lock() {
					let _ = chan.send(Event::Resize(term_width, term_height));
				}
			}
		}
	}

	let pixel_width = size.ws_xpixel as u32;
	let pixel_height = size.ws_ypixel as u32;

	if PIXEL_WIDTH.load(Ordering::SeqCst) != pixel_width || PIXEL_HEIGHT.load(Ordering::SeqCst) != pixel_height {
		PIXEL_WIDTH.store(pixel_width, Ordering::SeqCst);
		PIXEL_HEIGHT.store(pixel_height, Ordering::SeqCst);
		unsafe {
			if let Some(chan) = RESIZE_CHANNEL.as_ref() {
				if let Ok(chan) = chan.lock() {
					let _ = chan.send(Event::PixelResize(pixel_width, pixel_height));
				}
			}
		}
	}

	let mut font_width = (size.ws_xpixel.max(1) / size.ws_col.max(1)) as u32;
	let mut font_height = (size.ws_ypixel.max(1) / size.ws_row.max(1)) as u32;

	if font_width == 0 || font_height == 0 {
		font_width = 16;
		font_height = 32;
	}

	if FONT_WIDTH.load(Ordering::SeqCst) != font_width || FONT_HEIGHT.load(Ordering::SeqCst) != font_height {
		FONT_WIDTH.store(font_width, Ordering::SeqCst);
		FONT_HEIGHT.store(font_height, Ordering::SeqCst);
		unsafe {
			if let Some(chan) = RESIZE_CHANNEL.as_ref() {
				if let Ok(chan) = chan.lock() {
					let _ = chan.send(Event::FontResize(font_width, font_height));
				}
			}
		}
	}
}

impl Terminal {
	pub fn listen_for_resize(&mut self) {
		unsafe {
			if RESIZE_CHANNEL.is_none() {
				let (resize_tx, resize_rx) = mpsc::channel();
				RESIZE_CHANNEL = Some(Mutex::new(resize_tx));
				let senders_arc = self.event_channels.clone();
				// FIXME store thread handle somewhere
				thread::spawn(move || {
					while let Ok(event) = resize_rx.recv() {
						// Forward event to all listeners
						let senders = senders_arc.lock().unwrap();
						for tx in &*senders {
							tx.send(event.clone()).expect("Failed to send event");
						}
					}
				});
			}
			signal(SIGWINCH, handle_resize as sighandler_t);
		}
		self.refresh_size();
	}

	pub fn refresh_size(&self) {
		handle_resize(SIGWINCH);
	}

	pub fn width(&self) -> u32 {
		TERM_WIDTH.load(Ordering::SeqCst)
	}

	pub fn height(&self) -> u32 {
		TERM_HEIGHT.load(Ordering::SeqCst)
	}

	pub fn pixel_width(&self) -> u32 {
		PIXEL_WIDTH.load(Ordering::SeqCst)
	}

	pub fn pixel_height(&self) -> u32 {
		PIXEL_HEIGHT.load(Ordering::SeqCst)
	}

	pub fn font_width(&self) -> u32 {
		FONT_WIDTH.load(Ordering::SeqCst)
	}

	pub fn font_height(&self) -> u32 {
		FONT_HEIGHT.load(Ordering::SeqCst)
	}

	pub fn font_ratio(&self) -> f32 {
		self.font_height() as f32 / self.font_width() as f32
	}
}
