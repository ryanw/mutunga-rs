use super::Terminal;

#[allow(non_camel_case_types)]
pub struct termios;
#[allow(non_camel_case_types)]
type tcflag_t = u32;

impl Terminal {
	pub fn term_ios(&self) -> termios {
		termios
	}
	pub fn set_term_ios(&mut self, _ios: termios) {}
	pub fn set_iflag(&mut self, _flag: tcflag_t) {}
	pub fn unset_iflag(&mut self, _flag: tcflag_t) {}
	pub fn set_oflag(&mut self, _flag: tcflag_t) {}
	pub fn unset_oflag(&mut self, _flag: tcflag_t) {}
	pub fn set_cflag(&mut self, _flag: tcflag_t) {}
	pub fn unset_cflag(&mut self, _flag: tcflag_t) {}
	pub fn set_lflag(&mut self, _flag: tcflag_t) {}
	pub fn unset_lflag(&mut self, _flag: tcflag_t) {}
}
