use crossterm::{cursor::MoveTo, style::Print, ExecutableCommand, terminal::{Clear, ClearType}};
use std::io::{self, Write};
use std::ops::{Sub, Add, Div, Mul};
use crate::tmp::Matrix;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};


pub struct Game {
	graphic: Graphic2D,
}

impl Game {

	/// Run.
	pub fn run() {
		let mut game = Self::new();

		game.graphic.render();

		let q1 = Query::Material(1);
		let q2 = Query::DrawLine{ start: Coord2D(29, 0), end: Coord2D(29, 29) };

		game.graphic.update(q1);
		game.graphic.update(q2);

		game.game_loop();
	}

	/// New.
	pub fn new() -> Self {
		Game {
			graphic: Graphic2D::new(30, 30),
		}
	}
	
	/// Game loop.
	pub fn game_loop(&mut self) {
		loop {
			self.graphic.render()
		}
	}
}

pub struct Graphic2D {
	grid: Vec<Vec<u32>>,
	prev_grid: Vec<Vec<u32>>,
	curs: Cursor,
}

#[derive(Default)]
pub struct Cursor { pos: Coord2D, material: u32 }

impl Graphic2D {

	pub fn new(m: usize, n: usize) -> Self {
		Self {
			grid: vec![vec![0; n]; m],
			prev_grid: vec![vec![u32::MAX; n]; m],
			curs: Cursor::default(),
		}
	}

	pub fn update(&mut self, query: Query) {
		match query {
			Query::DrawLine { start, end } => {
				let vector = end - start;
				let len = (end - start).len() as i32;
				self.curs.pos = start;

				for k in 0..len {
					self.curs.pos = start + (vector * k) / len;
					self.cursor_paint();
				}
			},
			Query::Material(mat) => self.curs.material = mat,
		}
	}

	pub fn cursor_paint(&mut self) {
		let Coord2D(i, j) = self.curs.pos;
		let mat = self.curs.material;
		self.grid[i as usize][j as usize] = mat;
	}

	pub fn render(&mut self) {
		let mut stdout = io::stdout();
		
		for (y, row) in self.grid.iter().enumerate() {
			for (x, &cell) in row.iter().enumerate() {
				let current_char = match cell {
					0 => ' ',
					_ => 'o',
				};
				// Only print if different from previous frame.
				if self.prev_grid[y][x] != cell {
					stdout.execute(MoveTo(x as u16, y as u16)).unwrap();
					stdout.execute(Print(current_char)).unwrap();
				}
			}
    }

		stdout.flush().unwrap();
	}
}

pub enum Query {
	DrawLine { start: Coord2D, end: Coord2D },
	Material(u32),
}

#[derive(Clone, Copy, Default)]
pub struct Coord2D(i32, i32);
impl Coord2D {
	pub fn pythagorean_len(&self) -> u32 {
		(self.0.abs().pow(2) + self.1.abs().pow(2)).isqrt() as u32
	}
	pub fn len(&self) -> u32 {
		self.0.max(self.1) as u32
	}
}

impl Add for Coord2D {
	type Output = Self;
	fn add(self, other: Self) -> Self::Output {
		Coord2D(self.0 + other.0, self.1 + other.1)
	}
}
impl Sub for Coord2D {
	type Output = Self;
	fn sub(self, other: Self) -> Self::Output {
		Coord2D(self.0 - other.0, self.1 - other.1)
	}
}
impl Mul<i32> for Coord2D {
	type Output = Self;
	fn mul(self, scalar: i32) -> Self::Output {
		Coord2D(self.0 * scalar, self.1 * scalar)
	}
}
impl Div<i32> for Coord2D {
	type Output = Self;
	fn div(self, scalar: i32) -> Self::Output {
		Coord2D(self.0 / scalar, self.1 / scalar)
	}
}

