extern crate rand;

use std::fmt;
use std::io;
use std::process::exit;
use std::time::{Instant, Duration};
use rand::{thread_rng, Rng};

/* basic objects */

#[derive(Debug, PartialEq, Clone, Copy)]
struct Coords {
	x: usize,
	y: usize
}

impl Coords {
	fn new(x: usize, y: usize) -> Coords {
		Coords {
			x: x,
			y: y
		}
	}

	fn new_io(c1: &str, c2: &str, max: usize) -> Result<Coords, GameError> {
		let x0 = c1.parse();
		let y0 = c2.parse();

		if x0.is_err() || y0.is_err() { return Err(GameError::InvalidCoords) };

		let x = x0.unwrap();
		let y = y0.unwrap();

		if x <= 0 || y <= 0 || x > max || y > max {
			Err(GameError::InvalidCoords)
		} else {
			Ok(Coords::new(x, y))
		}
	}

	fn neighbors(&self, max: usize) -> Vec<Coords> {
		let mut neighbors = Vec::with_capacity(8);
		let (x, y) = (self.x, self.y);

		neighbors.push(Coords::new(x, y-1));
		neighbors.push(Coords::new(x, y+1));
		neighbors.push(Coords::new(x-1, y));
		neighbors.push(Coords::new(x+1, y));
		neighbors.push(Coords::new(x-1, y-1));
		neighbors.push(Coords::new(x-1, y+1));
		neighbors.push(Coords::new(x+1, y-1));
		neighbors.push(Coords::new(x+1, y+1));

		neighbors.into_iter().filter(|n| n.x > 0 && n.y > 0 && n.x <= max && n.y <= max).collect()
	}
}

#[derive(Debug, PartialEq)]
struct Field {
	coords: Coords,
	is_rigged: bool,
	is_flagged: bool,
	is_visible: bool
}

impl Field {
	fn new(coords: Coords, is_rigged: bool) -> Field {
		Field {
			coords: coords,
			is_rigged: is_rigged,
			is_flagged: false,
			is_visible: false
		}
	}

	fn show(&mut self) {
		self.is_visible = true;
	}
}

#[derive(Debug)]
struct Grid {
	size: usize,
	fields: Vec<Field>
}

impl Grid {
	fn new(size: usize, density: u32) -> Grid {
		let mut fields = Vec::with_capacity(size);
		let mut is_rigged;
		let mut rng = thread_rng();

		for x in 1..(size + 1) {
			for y in 1..(size + 1) {
				is_rigged = rng.gen_weighted_bool(10 - density * 2);
				fields.push(Field::new(Coords::new(x, y), is_rigged));
			}
		}

		Grid {
			size: size,
			fields: fields
		}
	}

	fn field_at(&self, coords: Coords) -> &Field {
		self.fields.iter().find(|f| f.coords == coords).unwrap()
	}

	fn field_at_mut(&mut self, coords: Coords) -> &mut Field {
		self.fields.iter_mut().find(|f| f.coords == coords).unwrap()
	}

	fn mines_around(&self, coords: Coords) -> usize {
		let mut n = 0;

		for c in coords.neighbors(self.size) {
			if self.field_at(c).is_rigged { n += 1 }
		}

		n
	}
}

// a hack to bypass the trait coherence rules
struct Time(Duration);

/* visuals */

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
			try!(write!(f, "{:2}", x));
			for y in 1..limit {
				let field = self.field_at(Coords::new(x, y));
				let disp = if field.is_flagged {
					"█".to_string()
				} else if !field.is_visible {
					"░".to_string()
				} else if field.is_rigged {
					"¤".to_string()
				} else {
					let n = self.mines_around(Coords::new(x, y));
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
		let mins = self.0.as_secs() / 60;
		let secs = self.0.as_secs() % 60;

		write!(f, "{}m{}s", mins, secs)
	}
}

/* the game */

struct Game {
	grid: Grid,
	uncovered: Vec<Coords>,
	buffer: String,
	time: Instant
}

enum GameAction {
	Resign,
	Uncover(Coords),
	Flag(Coords)
}

enum GameStatus {
	Proceed,
	GameOver,
	Victory
}

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
			uncovered: Vec::with_capacity(size^2),
			buffer: String::with_capacity(32),
			time: Instant::now()
		}
	}

	fn turn(&mut self) {
		println!("\n{}", self.grid);

		match self.check_status() {
			GameStatus::GameOver => {
				println!("wasted! time elapsed: {}", Time(self.time.elapsed()));
				exit(0)
			},
			GameStatus::Victory => {
				println!("victory! time elapsed: {}", Time(self.time.elapsed()));
				exit(0)
			},
			GameStatus::Proceed => ()
		}

		match self.handle_input() {
			Ok(a) => self.perform_action(a),
			Err(e) => self.print_error(e)
		}
	}

	fn check_status(&self) -> GameStatus {
		if let Some(_) = self.grid.fields.iter().find(|f| f.is_visible && f.is_rigged) {
			GameStatus::GameOver
		} else if self.uncovered.len() == self.grid.fields.iter().filter(|f| !f.is_rigged).collect::<Vec<_>>().len() {
			GameStatus::Victory
		} else {
			GameStatus::Proceed
		}
	}

	fn handle_input(&mut self) -> Result<GameAction, GameError> {
		self.buffer.clear();
		let _ = io::stdin().read_line(&mut self.buffer).unwrap();

		let words = self.buffer.split_whitespace().collect::<Vec<&str>>();
		if words.len() == 0 { return Err(GameError::InvalidInput) };

		if ["resign", "exit", "quit"].contains(&words[0]) {
			Ok(GameAction::Resign)
		} else {
			if words.len() != 3 { return Err(GameError::InvalidInput) };

			let coords = try!(Coords::new_io(&words[1], &words[2], self.grid.size));

			match words[0] {
				"uncover" | "u" => {
					if self.grid.field_at(coords).is_visible {
						Err(GameError::FieldAlreadyVisible)
					} else if self.grid.field_at(coords).is_flagged {
						Err(GameError::FieldIsFlagged)
					} else {
						Ok(GameAction::Uncover(coords))
					}
				},
				"flag" | "f" =>  {
					if self.grid.field_at(coords).is_visible {
						Err(GameError::FieldAlreadyVisible)
					} else {
						Ok(GameAction::Flag(coords))
					}
				},
				_ => Err(GameError::InvalidInput)
			}
		}
	}

	fn print_error(&self, error: GameError) {
		match error {
			GameError::InvalidCoords => println!("invalid coordinates!"),
			GameError::InvalidInput => println!("invalid command!"),
			GameError::FieldAlreadyVisible => println!("field already visible!"),
			GameError::FieldIsFlagged => println!("field is flagged!")
		}
	}

	fn perform_action(&mut self, action: GameAction) {
		match action {
			GameAction::Resign => { exit(0) },
			GameAction::Uncover(coords) => { self.uncover(coords) },
			GameAction::Flag(coords) => { self.flag(coords) }
		}
	}

	fn uncover(&mut self, coords: Coords) {
		self.grid.field_at_mut(coords).show();
		self.uncovered.push(coords);

		if !self.grid.field_at(coords).is_rigged && self.grid.mines_around(coords) == 0 {
			for c in coords.neighbors(self.grid.size) {
				if !self.grid.field_at(c).is_rigged && !self.uncovered.contains(&c) {
					self.uncover(c);
				}
			}
		}
	}

	fn flag(&mut self, coords: Coords) {
		if !self.grid.field_at(coords).is_visible {
			self.grid.field_at_mut(coords).is_flagged = !self.grid.field_at(coords).is_flagged;
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

#[test]
fn neighbor_listing() {
	assert_eq!(Coords::new(1, 1).neighbors(2).len(), 3);
	assert_eq!(Coords::new(1, 2).neighbors(2).len(), 3);
	assert_eq!(Coords::new(2, 1).neighbors(2).len(), 3);
	assert_eq!(Coords::new(2, 2).neighbors(2).len(), 3);
}