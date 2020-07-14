use super::Terminal;
use std::io;

type WriteResult = Result<(), io::Error>;

impl Terminal {
	pub fn normal_mode(&mut self) {
		self.enable_echo();
		self.enable_canonical();
		self.enable_flow_control();
		self.enable_input_processing();
		self.enable_cr_to_nl();
		self.enable_ctrl_c();
	}

	pub fn raw_mode(&mut self) {
		self.disable_echo();
		self.disable_canonical();
		self.disable_flow_control();
		self.disable_input_processing();
		self.disable_cr_to_nl();
		//self.disable_ctrl_c();
	}

	pub fn enable_echo(&mut self) {
		self.set_lflag(libc::ECHO);
	}

	pub fn disable_echo(&mut self) {
		self.unset_lflag(libc::ECHO);
	}

	pub fn enable_canonical(&mut self) {
		self.set_lflag(libc::ICANON);
	}

	pub fn disable_canonical(&mut self) {
		self.unset_lflag(libc::ICANON);
	}

	pub fn enable_flow_control(&mut self) {
		self.set_lflag(libc::IXON);
	}

	pub fn disable_flow_control(&mut self) {
		self.unset_lflag(libc::IXON);
	}

	pub fn enable_input_processing(&mut self) {
		self.set_lflag(libc::IEXTEN);
	}

	pub fn disable_input_processing(&mut self) {
		self.unset_lflag(libc::IEXTEN);
	}

	pub fn enable_cr_to_nl(&mut self) {
		self.set_lflag(libc::ICRNL);
	}

	pub fn disable_cr_to_nl(&mut self) {
		self.unset_lflag(libc::ICRNL);
	}

	pub fn enable_ctrl_c(&mut self) {
		self.set_lflag(libc::ISIG);
	}

	pub fn disable_ctrl_c(&mut self) {
		self.unset_lflag(libc::ISIG);
	}

	pub fn enable_mouse(&mut self) -> WriteResult {
		self.csi("?1000h")?; // Enable VT200
		self.csi("?1002h")?; // Enable "button" Xterm mouse events
					 //self.csi("?1015h")?; // Enable urxvt extended mouse positions (for > 223 cells)
		self.csi("?1006h")?; // Enable sgr extended mouse positions (for > 223 cells)
		Ok(())
	}

	pub fn disable_mouse(&mut self) -> WriteResult {
		self.csi("?1006l")?; // Disable sgr extended mouse positions (for > 223 cells)
					 //self.csi("?1015l")?; // Disble urxvt extended mouse positions (for > 223 cells)
		self.csi("?1002l")?; // Disable "button" Xterm mouse events
		self.csi("?1000l")?; // Disable VT200
		Ok(())
	}

	pub fn enable_mouse_move(&mut self) -> WriteResult {
		self.csi("?1003h")?; // Enable "any" Xterm mouse events (i.e. mouse move without buttons)
		Ok(())
	}

	pub fn disable_mouse_move(&mut self) -> WriteResult {
		self.csi("?1003l")?; // Enable "any" Xterm mouse events (i.e. mouse move without buttons)
		Ok(())
	}
}
