use super::Terminal;
use crate::Event;
use std::sync::mpsc;

impl Terminal {
	pub fn event_channel(&mut self) -> mpsc::Receiver<Event> {
		let (tx, rx) = mpsc::channel();
		let mut senders = self.event_channels.lock().unwrap();
		senders.push(tx);

		rx
	}
}
