#![allow(dead_code)]

use self::Rank::*;
use self::Suit::*;
use self::CardsError::*;
use std::fmt;
use rand::{thread_rng, Rng};

/* objects */

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Rank {
	Two = 2,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
	Nine,
	Ten,
	Jack,
	Queen,
	King,
	Ace,
	Joker
}

impl Rank {
	pub fn new_safe(rank: &str) -> Result<Rank, CardsError> {
		match &*rank.to_lowercase() {
			"ace" => Ok(Ace),
			"two" => Ok(Two),
			"three" => Ok(Three),
			"four" => Ok(Four),
			"five" => Ok(Five),
			"six" => Ok(Six),
			"seven" => Ok(Seven),
			"eight" => Ok(Eight),
			"nine" => Ok(Nine),
			"ten" => Ok(Ten),
			"jack" => Ok(Jack),
			"queen" => Ok(Queen),
			"king" => Ok(King),
			"joker" => Ok(Joker),
			_ => Err(InvalidRank)
		}
	}

	pub fn new(rank: &str) -> Rank { Rank::new_safe(rank).unwrap() }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Suit {
	Spades,
	Clubs,
	Diamonds,
	Hearts
}

impl Suit {
	pub fn new_safe(suit: &str) -> Result<Suit, CardsError> {
		match &*suit.to_lowercase() {
			"spades" => Ok(Spades),
			"clubs" => Ok(Clubs),
			"diamonds" => Ok(Diamonds),
			"hearts" => Ok(Hearts),
			_ => Err(InvalidSuit)
		}
	}

	pub fn new(suit: &str) -> Suit { Suit::new_safe(suit).unwrap() }
}

#[derive(Debug, PartialEq)]
pub struct Card {
	pub rank: Rank,
	pub suit: Option<Suit>
}

impl Card {
	pub fn new(rank: Rank, suit: Option<Suit>) -> Card {
		Card {
			rank: rank,
			suit: suit
		}
	}
}

pub struct Cards { pub cards: Vec<Card> }

pub type Deck = Cards;
pub type Hand = Cards;

impl Deck {
	pub fn new() -> Deck {
		let mut cards = Vec::with_capacity(53);
		for &suit in [Spades, Hearts, Diamonds, Clubs].iter() {
			for &rank in [Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace].iter() {
				cards.push(Card::new(rank, Some(suit)));
			}
		}

		Deck { cards: cards }
	}

	pub fn new_with_joker() -> Deck {
		let mut deck = Deck::new();
		deck.cards.push(Card::new(Joker, None));
		deck
	}

	pub fn shuffle(&mut self) {
		let mut rng = thread_rng();
		rng.shuffle(&mut self.cards);
	}

	pub fn size(&self) -> usize { self.cards.len() }

	pub fn draw1(&mut self) -> Result<Card, CardsError> {
		if self.size() == 0 {
			Err(EmptyDeck)
		} else {
			Ok(self.cards.pop().unwrap())
		}
	}

	pub fn draw(&mut self, number: usize) -> Vec<Card> {
		let mut drawn = Vec::with_capacity(number);

		for _ in 0..number { drawn.push(self.draw1().unwrap()) }

		drawn
	}
}

#[derive(Debug)]
pub enum CardsError {
	InvalidRank,
	InvalidSuit,
	EmptyDeck
}

/* displaying */

impl fmt::Display for Suit {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", match *self {
			Spades => "♠",
			Clubs => "♣",
			Diamonds => "♦",
			Hearts => "♥"
		})
	}
}

impl fmt::Display for Rank {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", match *self {
			Two => "2",
			Three => "3",
			Four => "4",
			Five => "5",
			Six => "6",
			Seven => "7",
			Eight => "8",
			Nine => "9",
			Ten => "10",
			Jack => "J",
			Queen => "Q",
			King => "K",
			Ace => "A",
			Joker => "Jkr"
		})
	}
}

impl fmt::Display for Card {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.rank == Joker {
			write!(f, "{}", self.rank)
		} else {
			write!(f, "{}{}", self.rank, self.suit.unwrap())
		}
	}
}

impl fmt::Display for Deck {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for (count, card) in self.cards.iter().enumerate() {
			if count != 0 { try!(write!(f, ", ")); }
			try!(write!(f, "{}", card));
		}
		write!(f, "")
	}
}