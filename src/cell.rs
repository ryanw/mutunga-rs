use crate::Color;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Cell {
	pub bg: Color,
	pub fg: Color,
	pub symbol: char,
}

impl Cell {
	pub fn null() -> Self {
		Self {
			fg: Color { r: 0, g: 0, b: 0, a: 0 },
			bg: Color { r: 0, g: 0, b: 0, a: 0 },
			symbol: '\0',
		}
	}

	pub fn is_null(&self) -> bool {
		Cell::null() == *self
	}

	pub fn blend(&self, cell: &Cell) -> Cell {
		let bg = self.bg.blend(&cell.bg);
		let fg;
		let symbol = if (self.symbol == ' ' || self.symbol == '\0') && cell.symbol != '\0' {
			fg = self.bg.blend(&cell.fg);
			cell.symbol
		} else {
			fg = self.fg.blend(&bg);
			self.symbol
		};

		Cell { fg, bg, symbol }
	}
}
