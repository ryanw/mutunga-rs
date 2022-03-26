use super::Terminal;
use std::io;

type WriteResult = Result<(), io::Error>;

impl Terminal {
	pub fn normal_mode(&mut self) {}
	pub fn raw_mode(&mut self) {}
	pub fn enable_echo(&mut self) {}
	pub fn disable_echo(&mut self) {}
	pub fn enable_canonical(&mut self) {}
	pub fn disable_canonical(&mut self) {}
	pub fn enable_flow_control(&mut self) {}
	pub fn disable_flow_control(&mut self) {}
	pub fn enable_input_processing(&mut self) {}
	pub fn disable_input_processing(&mut self) {}
	pub fn enable_cr_to_nl(&mut self) {}
	pub fn disable_cr_to_nl(&mut self) {}
	pub fn enable_ctrl_c(&mut self) {}
	pub fn disable_ctrl_c(&mut self) {}
	pub fn enable_mouse(&mut self) -> WriteResult {
		Ok(())
	}
	pub fn disable_mouse(&mut self) -> WriteResult {
		Ok(())
	}
	pub fn enable_mouse_move(&mut self) -> WriteResult {
		Ok(())
	}
	pub fn disable_mouse_move(&mut self) -> WriteResult {
		Ok(())
	}
}
