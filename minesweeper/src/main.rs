extern crate rand;

use std::fmt;
use std::io;
use std::ops::{Index, IndexMut};
use std::process::exit;
use std::time::{Instant, Duration};
use rand::{thread_rng, Rng};
use self::GameError::*;

/* basic objects */

#[derive(Debug, PartialEq, Clone, Copy)]
struct Coords {
	x: usize,
	y: usize
}

impl Coords {
	fn new(x: usize, y: usize) -> Coords {
		Coords { x: x, y: y }
	}

	fn new_io(c1: &str, c2: &str, max: usize) -> Result<Coords, GameError> {
		let x0 = c1.parse::<usize>();
		let y0 = c2.parse::<usize>();

		if x0.is_err() || y0.is_err() { return Err(InvalidCoords) };

		let x = x0.unwrap();
		let y = y0.unwrap();

		if x > max || y > max {
			Err(InvalidCoords)
		} else {
			Ok(Coords::new(x, y))
		}
	}

	fn neighbors(&self, max: usize) -> Vec<Coords> {
		let mut neighbors = Vec::with_capacity(8);

		for x in self.x-1..self.x+2 {
			for y in self.y-1..self.y+2 {
				if !(x == self.x && y == self.y) { neighbors.push(Coords::new(x, y)); }
			}
		}

		neighbors.into_iter().filter(|n| n.x > 0 && n.y > 0 && n.x <= max && n.y <= max).collect()
	}
}

impl From<(usize, usize)> for Coords {
    fn from((x, y): (usize, usize)) -> Self {
		Coords::new(x, y)
	}
}

#[derive(Debug, PartialEq)]
struct Field {
	coords: Coords,
	rigged: bool,
	flagged: bool,
	visible: bool
}

impl Field {
	fn new<T: Into<Coords>>(coords: T, rigged: bool) -> Field {
		Field {
			coords: coords.into(),
			rigged: rigged,
			flagged: false,
			visible: false
		}
	}

	fn flag(&mut self) -> Result<(), GameError> {
		if self.visible {
			Err(FieldAlreadyVisible)
		} else {
			self.flagged = !self.flagged;
			Ok(())
		}
	}
}

#[derive(Debug)]
struct Grid {
	size: usize,
	fields: Vec<Field>
}

impl<T: Into<Coords>+Copy> Index<T> for Grid {
    type Output = Field;

    fn index(&self, coords: T) -> &Field {
		self.fields.iter().find(|f| f.coords == coords.into()).expect("cannot borrow given field")
    }
}

impl<T: Into<Coords>+Copy> IndexMut<T> for Grid {
    fn index_mut(&mut self, coords: T) -> &mut Field {
		self.fields.iter_mut().find(|f| f.coords == coords.into()).expect("cannot mutably borrow given field")
    }
}

impl Grid {
	fn new(size: usize, density: u32) -> Grid {
		let mut fields = Vec::with_capacity(size);
		let mut rigged;
		let mut rng = thread_rng();

		for x in 1..(size + 1) {
			for y in 1..(size + 1) {
				rigged = rng.gen_weighted_bool(10 - density * 2);
				fields.push(Field::new((x, y), rigged));
			}
		}

		Grid {
			size: size,
			fields: fields
		}
	}

	fn mines_around<T: Into<Coords>>(&self, coords: T) -> usize {
		let neighboring_coords = coords.into().neighbors(self.size);
		self.fields.iter().filter(|f| neighboring_coords.contains(&f.coords) && f.rigged).count()
	}

	fn uncover<T: Into<Coords>+Copy>(&mut self, coords: T) -> Result<(), GameError> {
		if self[coords].visible {
			Err(FieldAlreadyVisible)
		} else if self[coords].flagged {
			Err(FieldIsFlagged)
		} else {
			self[coords].visible = true;
			if !self[coords].rigged && self.mines_around(coords) == 0 {
				for c in coords.into().neighbors(self.size) {
					if !self[c].rigged && !self[c].visible {
						let _ = self.uncover(c);
					}
				}
				Ok(())
			} else {
				Ok(())
			}
		}
	}
}

// a hack to bypass the trait coherence rules
struct Time(Duration);

/* displaying */

impl fmt::Display for Grid {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let limit = self.size + 1;
		let border_hor = '─';
		let border_ver = '│';
		let border_cross = '┼';

		// row line
		let mut row = String::new();
		row.push_str(&format!("{0}{0}{1}", border_hor, border_cross));
		for _ in 1..limit {
			row.push_str(&format!("{0}{0}{0}{1}", border_hor, border_cross));
		}
		row.push_str(&format!("{0}{0}", border_hor));

		// top numbers
		try!(write!(f, "  "));
		for n in 1..limit {
			try!(write!(f, "|{:^3}", n));
		}
		try!(writeln!(f, "|"));

		// fields
		for x in 1..limit {
			try!(writeln!(f, "{}", row));
			try!(write!(f, "{:<2}", x));
			for y in 1..limit {
				let field = &self[(x, y)];
				let disp = if field.flagged {
					"█".to_string()
				} else if !field.visible {
					"░".to_string()
				} else if field.rigged {
					"¤".to_string()
				} else {
					let n = self.mines_around((x, y));
					if n == 0 {
						" ".to_string()
					} else {
						format!("{}", n)
					}
				};
				try!(write!(f, "{} {} ", border_ver, disp));
			}
			try!(writeln!(f, "{}{:2}", border_ver, x));
		}
		try!(writeln!(f, "{}", row));

		// bottom numbers
		try!(write!(f, "  "));
		for n in 1..limit {
			try!(write!(f, "|{:^3}", n));
		}
		try!(writeln!(f, "|"));

		write!(f, "")
	}
}

impl fmt::Display for Time {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}m{}s", self.0.as_secs() / 60, self.0.as_secs() % 60)
	}
}

impl fmt::Display for GameError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			InvalidCoords => write!(f, "invalid coordinates!"),
			InvalidInput => write!(f, "invalid command!"),
			FieldAlreadyVisible => write!(f, "field already visible!"),
			FieldIsFlagged => write!(f, "field is flagged!")
		}
	}
}

/* the game */

struct Game {
	grid: Grid,
	buffer: String,
	time: Instant
}

enum GameStatus {
	Proceed,
	GameOver,
	Victory
}

#[derive(Debug, PartialEq)]
enum GameError {
	InvalidInput,
	InvalidCoords,
	FieldAlreadyVisible,
	FieldIsFlagged
}

impl Game {
	fn new(size: usize, difficulty: u32) -> Game {
		Game {
			grid: Grid::new(size, difficulty),
			buffer: String::with_capacity(32),
			time: Instant::now()
		}
	}

	fn turn(&mut self) {
		println!("\n{}", self.grid);

		match self.check_status() {
			GameStatus::GameOver | GameStatus::Victory => {
				println!("game over! time elapsed: {}", Time(self.time.elapsed()));
				let _ = io::stdin().read_line(&mut self.buffer).unwrap();
				exit(0)
			},
			GameStatus::Proceed => ()
		}

		if let Err(e) = self.handle_input() { println!("{}", e) }
	}

	fn check_status(&self) -> GameStatus {
		if let Some(_) = self.grid.fields.iter().find(|f| f.visible && f.rigged) {
			GameStatus::GameOver
		} else if self.grid.fields.iter().filter(|f| f.visible).count() == self.grid.fields.iter().filter(|f| !f.rigged).count() {
			GameStatus::Victory
		} else {
			GameStatus::Proceed
		}
	}

	fn handle_input(&mut self) -> Result<(), GameError> {
		self.buffer.clear();
		let _ = io::stdin().read_line(&mut self.buffer).unwrap();

		let tmp_buffer = self.buffer.clone();
		let words = tmp_buffer.split_whitespace().collect::<Vec<&str>>();

		if words.len() == 0 {
			Err(InvalidInput)
		} else if ["resign", "exit", "quit"].contains(&words[0]) {
			exit(0)
		} else {
			if words.len() != 3 { return Err(InvalidInput) };

			let coords = try!(Coords::new_io(&words[1], &words[2], self.grid.size));

			match words[0] {
				"uncover" | "u" => self.grid.uncover(coords),
				"flag" | "f"	=> self.grid[coords].flag(),
				_				=> Err(InvalidInput)
			}
		}
	}
}

/* game */

fn play() {
	let mut game = Game::new(10, 1);

	loop { game.turn() }
}

fn main() {
	play()
}

/* tests */

#[cfg(test)]
mod test {
	use super::Coords;
	use super::Field;
	use super::Grid;
	use super::GameError::*;

	fn staged_grid(size: usize, mines: &[Coords]) -> Grid {
		let mut fields = Vec::with_capacity(size);

		for x in 1..(size + 1) {
			for y in 1..(size + 1) {
				fields.push(Field::new((x, y), mines.contains(&Coords::new(x, y))));
			}
		}

		Grid {
			size: size,
			fields: fields
		}
	}

	#[test]
	fn creating_coords() {
		assert_eq!(Ok(Coords::new( 1,  1)), Coords::new_io( "1",  "1", 10));
		assert_eq!(Ok(Coords::new(10, 10)), Coords::new_io("10", "10", 10));
		assert_eq!(Err(InvalidCoords), Coords::new_io("11", "10", 10));
		assert_eq!(Err(InvalidCoords), Coords::new_io("10", "11", 10));
		assert_eq!(Err(InvalidCoords), Coords::new_io( "1",   "", 10));
		assert_eq!(Err(InvalidCoords), Coords::new_io(  "",  "1", 10));
		assert_eq!(Err(InvalidCoords), Coords::new_io(  "",   "", 10));
		assert_eq!(Err(InvalidCoords), Coords::new_io("1a",  "1", 10));
		assert_eq!(Err(InvalidCoords), Coords::new_io("a1",  "1", 10));
		assert_eq!(Err(InvalidCoords), Coords::new_io( "a",  "1", 10));
	}

	#[test]
	fn coord_neighbors() {
		assert_eq!(Coords::new(1, 1).neighbors(2).len(), 3);
		assert_eq!(Coords::new(1, 2).neighbors(2).len(), 3);
		assert_eq!(Coords::new(2, 1).neighbors(2).len(), 3);
		assert_eq!(Coords::new(2, 2).neighbors(2).len(), 3);
		assert_eq!(Coords::new(1, 2).neighbors(3).len(), 5);
		assert_eq!(Coords::new(2, 1).neighbors(3).len(), 5);
		assert_eq!(Coords::new(2, 2).neighbors(3).len(), 8);
		assert_eq!(Coords::new(2, 3).neighbors(3).len(), 5);
		assert_eq!(Coords::new(3, 2).neighbors(3).len(), 5);
	}

	#[test]
	fn indexing_grid() {
		let grid = staged_grid(10, &[Coords::new(1, 1), Coords::new(5, 5), Coords::new(10, 10)]);

		assert!(&grid[( 1,  1)].rigged);
		assert!(&grid[( 5,  5)].rigged);
		assert!(&grid[(10, 10)].rigged);
	}

	#[test]
	fn flagging() {
		let mut f = Field::new((1, 1), true);
		assert_eq!(f.flagged, false);
		let _ = f.flag();
		assert_eq!(f.flagged, true);
		let _ = f.flag();
		assert_eq!(f.flagged, false);
	}

	#[test]
	fn counting_mines() {
		let grid = staged_grid(3, &[Coords::new(1, 1), Coords::new(3, 3)]);

		assert_eq!(grid.mines_around((1, 1)), 0);
		assert_eq!(grid.mines_around((1, 2)), 1);
		assert_eq!(grid.mines_around((1, 3)), 0);
		assert_eq!(grid.mines_around((2, 1)), 1);
		assert_eq!(grid.mines_around((2, 2)), 2);
		assert_eq!(grid.mines_around((2, 3)), 1);
		assert_eq!(grid.mines_around((3, 1)), 0);
		assert_eq!(grid.mines_around((3, 2)), 1);
		assert_eq!(grid.mines_around((3, 3)), 0);
	}

	#[test]
	fn uncovering_tight_space() {
		let mut grid = staged_grid(3, &[Coords::new(2, 1)]);
		let _ = grid.uncover((1, 1));

		assert!( &grid[(1, 1)].visible);
		assert!(!&grid[(1, 2)].visible);
		assert!(!&grid[(1, 3)].visible);
		assert!(!&grid[(2, 1)].visible);
		assert!(!&grid[(2, 2)].visible);
		assert!(!&grid[(2, 3)].visible);
		assert!(!&grid[(3, 1)].visible);
		assert!(!&grid[(3, 2)].visible);
		assert!(!&grid[(3, 3)].visible);
	}

	#[test]
	fn uncovering_open_space() {
		let mut grid = staged_grid(3, &[Coords::new(3, 3)]);
		let _ = grid.uncover((1, 1));

		assert!( &grid[(1, 1)].visible);
		assert!( &grid[(1, 2)].visible);
		assert!( &grid[(1, 3)].visible);
		assert!( &grid[(2, 1)].visible);
		assert!( &grid[(2, 2)].visible);
		assert!( &grid[(2, 3)].visible);
		assert!( &grid[(3, 1)].visible);
		assert!( &grid[(3, 2)].visible);
		assert!(!&grid[(3, 3)].visible);
	}
}