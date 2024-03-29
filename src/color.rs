use std::mem;

pub type Ansi8BitColor = u8;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Color {
	pub a: u8,
	pub r: u8,
	pub g: u8,
	pub b: u8,
}

impl From<f32> for Color {
	fn from(grey: f32) -> Color {
		let g = (grey * 255.0) as u8;
		Color::grey(g)
	}
}

impl From<f64> for Color {
	fn from(grey: f64) -> Color {
		let g = (grey * 255.0) as u8;
		Color::grey(g)
	}
}

impl From<(f32, f32, f32, f32)> for Color {
	fn from(color: (f32, f32, f32, f32)) -> Color {
		Color::rgba(
			(color.0 * 255.0) as u8,
			(color.1 * 255.0) as u8,
			(color.2 * 255.0) as u8,
			(color.3 * 255.0) as u8,
		)
	}
}

impl From<String> for Color {
	fn from(hex: String) -> Color {
		Color::from(hex.as_str())
	}
}

impl From<&str> for Color {
	fn from(mut hex: &str) -> Color {
		if hex.len() == 0 {
			return Color::black();
		}
		if hex.chars().nth(0).unwrap() == '#' {
			hex = &hex[1..];
		}
		if hex.len() != 6 && hex.len() != 8 {
			return Color::black();
		}

		let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0x00);
		let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0x00);
		let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0x00);
		let a = if hex.len() == 8 {
			u8::from_str_radix(&hex[6..8], 16).unwrap_or(0x00)
		} else {
			0xff
		};

		Color::rgba(r, g, b, a)
	}
}

impl From<u32> for Color {
	fn from(color: u32) -> Color {
		unsafe { mem::transmute(color) }
	}
}

impl From<u64> for Color {
	fn from(color: u64) -> Color {
		(color as u32).into()
	}
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
	pub fn hsl(h: f32, s: f32, l: f32) -> Self {
		if s == 0.0 {
			return Color::grey((l * 255.0) as u8);
		}

		let q = if l < 0.5 { l * (1.0 + s) } else { l + s - l * s };
		let p = 2.0 * l - q;

		let r = hue_to_rgb(p, q, h + 1.0 / 3.0) * 255.0;
		let g = hue_to_rgb(p, q, h) * 255.0;
		let b = hue_to_rgb(p, q, h - 1.0 / 3.0) * 255.0;
		Color::rgb(r as u8, g as u8, b as u8)
	}

	pub fn rgb(r: u8, g: u8, b: u8) -> Self {
		Self::rgba(r, g, b, 255)
	}

	pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
		Self { r, g, b, a }
	}

	pub fn grey(val: u8) -> Self {
		Self::rgb(val, val, val)
	}

	pub fn gray(val: u8) -> Self {
		Self::grey(val)
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

	pub fn as_argb(&self) -> (u8, u8, u8, u8) {
		(self.a, self.r, self.g, self.b)
	}

	pub fn as_abgr(&self) -> (u8, u8, u8, u8) {
		(self.a, self.b, self.g, self.r)
	}

	pub fn as_bgra(&self) -> (u8, u8, u8, u8) {
		(self.b, self.g, self.r, self.a)
	}

	pub fn as_floats(&self) -> (f32, f32, f32, f32) {
		(
			// About 15% faster than / 255.0
			self.r as f32 * (1.0 / 255.0),
			self.g as f32 * (1.0 / 255.0),
			self.b as f32 * (1.0 / 255.0),
			self.a as f32 * (1.0 / 255.0),
		)
	}

	pub fn as_f64(&self) -> (f64, f64, f64, f64) {
		(
			self.r as f64 * (1.0 / 255.0),
			self.g as f64 * (1.0 / 255.0),
			self.b as f64 * (1.0 / 255.0),
			self.a as f64 * (1.0 / 255.0),
		)
	}

	pub fn as_u32(&self) -> u32 {
		unsafe { mem::transmute(*self) }
	}

	pub fn as_rgb_hex(&self) -> String {
		format!("{:02x?}{:02x?}{:02x?}", self.r, self.g, self.b)
	}

	pub fn as_rgba_hex(&self) -> String {
		format!("{:02x?}{:02x?}{:02x?}{:02x?}", self.r, self.g, self.b, self.a)
	}

	pub fn blend(&self, bg: &Color) -> Color {
		if self.a == 0xff {
			return self.clone();
		}
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

fn hue_to_rgb(p: f32, q: f32, mut t: f32) -> f32 {
	if t < 0.0 {
		t += 1.0;
	}
	if t > 1.0 {
		t -= 1.0;
	}

	if t < 1.0 / 6.0 {
		return p + (q - p) * 6.0 * t;
	}

	if t < 1.0 / 2.0 {
		return q;
	}

	if t < 2.0 / 3.0 {
		return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
	}

	return p;
}
