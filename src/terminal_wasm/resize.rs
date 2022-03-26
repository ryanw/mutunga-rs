use super::Terminal;

impl Terminal {
	pub fn listen_for_resize(&mut self) {}

	pub fn refresh_size(&self) {}

	pub fn width(&self) -> u32 {
		self.width
	}

	pub fn height(&self) -> u32 {
		self.height
	}

	pub fn pixel_width(&self) -> u32 {
		self.width * self.font_width()
	}

	pub fn pixel_height(&self) -> u32 {
		self.width * self.font_height()
	}

	pub fn font_width(&self) -> u32 {
		self.font_width
	}

	pub fn font_height(&self) -> u32 {
		self.font_height
	}

	pub fn font_ratio(&self) -> f32 {
		self.font_height() as f32 / self.font_width() as f32
	}
}
