pub type Ansi8BitColor = u8;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Color {
	pub r: u8,
	pub g: u8,
	pub b: u8,
	pub a: u8,
}

impl From<(u8, u8, u8)> for Color {
	fn from(color: (u8, u8, u8)) -> Color {
		Color::rgb(color.0, color.1, color.2)
	}
}

impl From<[u8; 3]> for Color {
	fn from(color: [u8; 3]) -> Color {
		Color::rgb(color[0], color[1], color[2])
	}
}

impl From<(u8, u8, u8, u8)> for Color {
	fn from(color: (u8, u8, u8, u8)) -> Color {
		Color::rgba(color.0, color.1, color.2, color.3)
	}
}

impl From<[u8; 4]> for Color {
	fn from(color: [u8; 4]) -> Color {
		Color::rgba(color[0], color[1], color[2], color[3])
	}
}

impl Color {
	pub fn rgb(r: u8, g: u8, b: u8) -> Self {
		Self::rgba(r, g, b, 255)
	}

	pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
		Self { r, g, b, a }
	}

	pub fn transparent() -> Self {
		Self { r: 0, g: 0, b: 0, a: 0 }
	}

	pub fn black() -> Self {
		Self {
			r: 0,
			g: 0,
			b: 0,
			a: 255,
		}
	}

	pub fn white() -> Self {
		Self {
			r: 255,
			g: 255,
			b: 255,
			a: 255,
		}
	}

	pub fn red() -> Self {
		Self {
			r: 255,
			g: 0,
			b: 0,
			a: 255,
		}
	}

	pub fn green() -> Self {
		Self {
			r: 0,
			g: 255,
			b: 0,
			a: 255,
		}
	}

	pub fn blue() -> Self {
		Self {
			r: 0,
			g: 0,
			b: 255,
			a: 255,
		}
	}

	pub fn yellow() -> Self {
		Self {
			r: 255,
			g: 255,
			b: 0,
			a: 255,
		}
	}

	pub fn magenta() -> Self {
		Self {
			r: 255,
			g: 0,
			b: 255,
			a: 255,
		}
	}

	pub fn cyan() -> Self {
		Self {
			r: 0,
			g: 255,
			b: 255,
			a: 255,
		}
	}

	pub fn as_8bit_ansi(&self) -> Ansi8BitColor {
		let r = self.r / 51;
		let g = self.g / 51;
		let b = self.b / 51;

		16 + 36 * r + 6 * g + b
	}

	pub fn as_rgb(&self) -> (u8, u8, u8) {
		(self.r, self.g, self.b)
	}

	pub fn as_rgba(&self) -> (u8, u8, u8, u8) {
		(self.r, self.g, self.b, self.a)
	}

	pub fn as_floats(&self) -> (f32, f32, f32, f32) {
		(
			self.r as f32 / 255.0,
			self.g as f32 / 255.0,
			self.b as f32 / 255.0,
			self.a as f32 / 255.0,
		)
	}

	pub fn blend(&self, bg: &Color) -> Color {
		let (fg_r, fg_g, fg_b, fg_a) = self.as_floats();
		let (bg_r, bg_g, bg_b, bg_a) = bg.as_floats();

		let a = (1.0 - fg_a) * bg_a + fg_a;
		let r = ((1.0 - fg_a) * bg_a * bg_r + fg_a * fg_r) / a;
		let g = ((1.0 - fg_a) * bg_a * bg_g + fg_a * fg_g) / a;
		let b = ((1.0 - fg_a) * bg_a * bg_b + fg_a * fg_b) / a;
		Color::rgba(
			(r * 255.0) as u8,
			(g * 255.0) as u8,
			(b * 255.0) as u8,
			(a * 255.0) as u8,
		)
	}

	pub fn set_brightness(&mut self, brightness: f32) {
		self.r = (self.r as f32 * brightness) as u8;
		self.g = (self.g as f32 * brightness) as u8;
		self.b = (self.b as f32 * brightness) as u8;
	}
}
