use super::Terminal;
use libc::{tcflag_t, tcgetattr, tcsetattr, termios, STDIN_FILENO, TCSANOW};

impl Terminal {
	pub fn term_ios(&self) -> termios {
		let mut ios = termios {
			// Input flags
			c_iflag: Default::default(),
			// Output flags
			c_oflag: Default::default(),
			// Control flags
			c_cflag: Default::default(),
			// Local flags
			c_lflag: Default::default(),
			#[cfg(target_os = "linux")]
			c_line: Default::default(),
			c_cc: Default::default(),
			#[cfg(not(target_env = "musl"))]
			c_ispeed: Default::default(),
			#[cfg(target_env = "musl")]
			__c_ispeed: Default::default(),
			#[cfg(not(target_env = "musl"))]
			c_ospeed: Default::default(),
			#[cfg(target_env = "musl")]
			__c_ospeed: Default::default(),
		};

		unsafe {
			tcgetattr(STDIN_FILENO, &mut ios);
		}

		ios
	}

	pub fn set_term_ios(&mut self, ios: termios) {
		unsafe {
			tcsetattr(STDIN_FILENO, TCSANOW, &ios);
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
