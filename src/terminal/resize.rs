use crate::Event;
use libc::{c_int, sighandler_t, signal, syscall, winsize, SYS_ioctl, SIGWINCH, STDIN_FILENO, TIOCGWINSZ};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{mpsc, Mutex};
use std::thread;

use super::Terminal;

static TERM_WIDTH: AtomicU32 = AtomicU32::new(0);
static TERM_HEIGHT: AtomicU32 = AtomicU32::new(0);
static mut RESIZE_CHANNEL: Option<Mutex<mpsc::Sender<Event>>> = None;

extern "C" fn handle_resize(_sig: c_int) {
	let mut size = winsize {
		ws_row: 0,
		ws_col: 0,
		ws_xpixel: 0,
		ws_ypixel: 0,
	};
	unsafe {
		syscall(SYS_ioctl, STDIN_FILENO, TIOCGWINSZ, &mut size);
	}
	let width = size.ws_col as u32;
	let height = size.ws_row as u32;
	TERM_WIDTH.store(width, Ordering::SeqCst);
	TERM_HEIGHT.store(height, Ordering::SeqCst);
	unsafe {
		if let Some(chan) = RESIZE_CHANNEL.as_ref() {
			chan.lock()
				.unwrap()
				.send(Event::Resize(width, height))
				.expect("Failed to send resize event");
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
}
