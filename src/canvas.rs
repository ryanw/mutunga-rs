use crate::{geom::Rect, Cell, Color};

#[derive(Default, Clone, Debug)]
pub struct Canvas {
	width: u32,
	height: u32,
	cells: Vec<Cell>,
}

pub struct CanvasRegion<'a> {
	pub(crate) canvas: &'a mut Canvas,
	pub(crate) rect: Rect,
}

impl<'a> CanvasRegion<'a> {
	pub fn region_mut(&mut self, rect: Rect) -> CanvasRegion {
		CanvasRegion {
			canvas: self.canvas,
			rect: Rect::new(rect.x + self.rect.x, rect.y + self.rect.y, rect.width, rect.height),
		}
	}

	pub fn width(&self) -> u32 {
		self.rect.width as u32
	}

	pub fn height(&self) -> u32 {
		self.rect.height as u32
	}

	pub fn size(&self) -> (u32, u32) {
		(self.width(), self.height())
	}

	pub fn clear(&mut self) {
		self.fill(Cell::null());
	}

	pub fn fill(&mut self, cell: Cell) {
		self.fill_rect(cell, &Rect::new(0, 0, self.rect.width, self.rect.height));
	}

	pub fn fill_rect(&mut self, cell: Cell, rect: &Rect) {
		let mut rect = rect.clone();
		rect.x += self.rect.x;
		rect.y += self.rect.y;
		self.canvas.fill_rect(cell, &rect);
	}
}

impl Canvas {
	pub fn new(width: u32, height: u32) -> Self {
		let mut canvas = Canvas::default();
		canvas.resize(width, height);
		canvas
	}

	pub fn region_mut(&mut self, rect: Rect) -> CanvasRegion {
		CanvasRegion { canvas: self, rect }
	}

	pub fn as_region_mut(&mut self) -> CanvasRegion {
		self.region_mut(Rect::new(0, 0, self.width as i32, self.height as i32))
	}

	pub fn clear(&mut self) {
		self.fill(Cell::null());
	}

	pub fn fill(&mut self, cell: Cell) {
		self.cells = vec![cell; self.width as usize * self.height as usize];
	}

	pub fn fill_rect(&mut self, new_cell: Cell, rect: &Rect) {
		for y in 0..rect.height {
			for x in 0..rect.width {
				let px = x + rect.x;
				let py = y + rect.y;
				if let Some(cell) = self.cell_mut(px, py) {
					*cell = new_cell.clone();
				}
			}
		}
	}

	pub fn width(&self) -> u32 {
		self.width
	}

	pub fn height(&self) -> u32 {
		self.height
	}

	pub fn size(&self) -> (u32, u32) {
		(self.width, self.height)
	}

	pub fn resize(&mut self, w: u32, h: u32) {
		self.width = w;
		self.height = h;
		self.cells = vec![Cell::null(); w as usize * h as usize];
	}

	pub fn index(&self, x: i32, y: i32) -> Option<usize> {
		if x < 0 || y < 0 || x as u32 >= self.width || y as u32 >= self.height {
			return None;
		}

		Some(x as usize + y as usize * self.width as usize)
	}

	pub fn cell(&self, x: i32, y: i32) -> Option<&Cell> {
		if let Some(idx) = self.index(x, y) {
			Some(&self.cells[idx])
		} else {
			None
		}
	}

	pub fn cell_mut(&mut self, x: i32, y: i32) -> Option<&mut Cell> {
		if let Some(idx) = self.index(x, y) {
			Some(&mut self.cells[idx])
		} else {
			None
		}
	}

	pub fn draw_text(&mut self, mut x: i32, y: i32, fg: Color, bg: Color, text: &str) {
		for symbol in text.chars() {
			if let Some(dst) = self.cell_mut(x, y) {
				let cell = Cell { fg, bg, symbol };
				*dst = cell.blend(dst);
			}
			x += 1;
		}
	}

	pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, cell: Cell) {
		let mut xs = x0 as f32;
		let mut ys = y0 as f32;
		let xe = x1 as f32;
		let ye = y1 as f32;

		let xd = (xe - xs).abs();
		let yd = (ye - ys).abs();

		let xc = if x0 < x1 { 1.0 } else { -1.0 };

		let yc = if y0 < y1 { 1.0 } else { -1.0 };

		let mut err = if xd >= yd { xd / 2.0 } else { -yd / 2.0 };

		loop {
			if let Some(dst) = self.cell_mut(xs as i32, ys as i32) {
				*dst = cell.blend(dst);
			}

			if xs == xe && ys == ye {
				break;
			}

			let err2 = err;
			if err2 > -xd {
				err -= yd;
				xs += xc;
			}
			if err2 < yd {
				err += xd;
				ys += yc;
			}
		}
	}

	pub fn draw_canvas(&mut self, dx: i32, dy: i32, canvas: &Canvas) {
		let mut width = canvas.width() as i32;
		let mut height = canvas.height() as i32;
		if width + dx > self.width() as i32 {
			width = self.width() as i32 - dx;
		}
		if height + dy > self.height() as i32 {
			height = self.height() as i32 - dy;
		}
		for y in 0..height {
			for x in 0..width {
				if let Some(dst_cell) = self.cell_mut(x + dx, y + dy) {
					if let Some(src_cell) = canvas.cell(x, y) {
						*dst_cell = src_cell.blend(dst_cell);
					}
				}
			}
		}
	}
}
