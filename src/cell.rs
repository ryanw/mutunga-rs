use crate::Color;
use phf::phf_map;

// Assign 1 bit per each of the 4 sides of a char
// Set bit if the char touches that edge
const TOP: u8 = 1 << 0;
const RIGHT: u8 = 1 << 1;
const BOTTOM: u8 = 1 << 2;
const LEFT: u8 = 1 << 3;

static CHAR_TO_EDGES: phf::Map<char, u8> = phf_map! {
	' ' => 0,
	'╵' => TOP,
	'╶' => RIGHT,
	'╷' => BOTTOM,
	'╴' => LEFT,
	'─' => RIGHT | LEFT,
	'│' => TOP | BOTTOM,
	'┼' => TOP | RIGHT | BOTTOM |LEFT,
	'┌' => RIGHT | BOTTOM,
	'┘' => TOP | LEFT,
	'┐' => BOTTOM | LEFT,
	'└' => TOP | RIGHT,
	'┴' => TOP | RIGHT | LEFT,
	'┤' => TOP | BOTTOM | LEFT,
	'┬' => RIGHT | BOTTOM | LEFT,
	'├' => TOP | RIGHT | BOTTOM,
};

static EDGES_TO_CHAR: phf::Map<u8, char> = phf_map! {
// FIXME use consts somehow - phf_map doesn't like them
	0_u8 => ' ',
	1_u8 => '╵',
	2_u8 => '╶',
	3_u8=> '└',
	4_u8 => '╷',
	5_u8=> '│',
	6_u8=> '┌',
	7_u8=> '├',
	8_u8 => '╴',
	9_u8=> '┘',
	10_u8 => '─',
	11_u8 => '┴',
	12_u8 => '┐',
	13_u8 => '┤',
	14_u8 => '┬',
	15_u8 => '┼',
};

fn get_edges(c: char) -> u8 {
	if c < '─' {
		return 0;
	}
	*CHAR_TO_EDGES.get(&c).unwrap_or(&0)
}

fn get_char(edges: u8, default: char) -> char {
	if edges == 0 {
		return default;
	}
	*EDGES_TO_CHAR.get(&edges).unwrap_or(&default)
}

fn blend_symbol(front: char, back: char) -> char {
	if front == ' ' || (front == '\0' && back != '\0') {
		return back;
	}
	let c0 = get_edges(front);
	if c0 == 0 {
		return front;
	}
	let c1 = get_edges(back);
	if c1 == 0 {
		return front;
	}

	get_char(c0 | c1, front)
}

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
		let mut fg = self.fg.clone();
		let mut symbol = self.symbol;

		if (self.symbol == ' ' || self.symbol == '\0') && cell.symbol != '\0' {
			fg = self.bg.blend(&cell.fg);
			symbol = cell.symbol
		} else if self.bg.a < 250 {
			// If semi-transparent, blend symbols
			fg = fg.blend(&bg);
			symbol = blend_symbol(self.symbol, cell.symbol)
		} else {
			fg = fg.blend(&bg);
		};

		Cell { fg, bg, symbol }
	}
}
