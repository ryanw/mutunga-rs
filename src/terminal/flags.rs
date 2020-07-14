use super::Terminal;
use libc::{syscall, tcflag_t, termios, SYS_ioctl, STDIN_FILENO, TCGETS, TCSETS};

impl Terminal {
	pub fn term_ios(&self) -> termios {
		let mut ios = termios {
			c_iflag: Default::default(),
			c_oflag: Default::default(),
			c_cflag: Default::default(),
			c_lflag: Default::default(),
			c_line: Default::default(),
			c_cc: Default::default(),
			c_ispeed: Default::default(),
			c_ospeed: Default::default(),
		};

		unsafe {
			syscall(SYS_ioctl, STDIN_FILENO, TCGETS, &mut ios);
		}

		ios
	}

	pub fn set_term_ios(&mut self, ios: termios) {
		unsafe {
			syscall(SYS_ioctl, STDIN_FILENO, TCSETS, &ios);
		}
	}

	pub fn set_iflag(&mut self, flag: tcflag_t) {
		let mut ios = self.term_ios();
		ios.c_iflag |= flag;
		self.set_term_ios(ios);
	}

	pub fn unset_iflag(&mut self, flag: tcflag_t) {
		let mut ios = self.term_ios();
		ios.c_iflag &= !flag;
		self.set_term_ios(ios);
	}

	pub fn set_oflag(&mut self, flag: tcflag_t) {
		let mut ios = self.term_ios();
		ios.c_oflag |= flag;
		self.set_term_ios(ios);
	}

	pub fn unset_oflag(&mut self, flag: tcflag_t) {
		let mut ios = self.term_ios();
		ios.c_oflag &= !flag;
		self.set_term_ios(ios);
	}

	pub fn set_cflag(&mut self, flag: tcflag_t) {
		let mut ios = self.term_ios();
		ios.c_cflag |= flag;
		self.set_term_ios(ios);
	}

	pub fn unset_cflag(&mut self, flag: tcflag_t) {
		let mut ios = self.term_ios();
		ios.c_cflag &= !flag;
		self.set_term_ios(ios);
	}

	pub fn set_lflag(&mut self, flag: tcflag_t) {
		let mut ios = self.term_ios();
		ios.c_lflag |= flag;
		self.set_term_ios(ios);
	}

	pub fn unset_lflag(&mut self, flag: tcflag_t) {
		let mut ios = self.term_ios();
		ios.c_lflag &= !flag;
		self.set_term_ios(ios);
	}
}
