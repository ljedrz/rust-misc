use term::*;
use term::{Term, Error};
use term::Term::*;
use term::Error::*;
use booleans::*;

// PAIR := λxyf.f x y
pub fn pair() -> Term { abs(abs(abs(Var(1).app(Var(3)).app(Var(2))))) }

// FIRST := λp.p TRUE
pub fn fst() -> Term { abs(Var(1).app(tru())) }

// SECOND := λp.p FALSE
pub fn snd() -> Term { abs(Var(1).app(fls())) }

// NIL := λx.TRUE
pub fn nil() -> Term { abs(tru()) }

// NULL := λp.p (λxy.FALSE)
pub fn is_empty() -> Term { abs(Var(1).app(abs(abs(fls())))) }

// CONS := λht.PAIR FALSE (PAIR h t)
pub fn cons() -> Term { abs(abs(pair().app(fls()).app(pair().app(Var(2)).app(Var(1))))) }

// HEAD := λz.FIRST (SECOND z)
pub fn head() -> Term { abs(fst().app(snd().app(Var(1)))) }

// TAIL := λz.SECOND (SECOND z)
pub fn tail() -> Term { abs(snd().app(snd().app(Var(1)))) }

impl Term {
	pub fn is_pair(&self) -> bool {
		self.fst_ref().is_ok() && self.snd_ref().is_ok()
	}

	pub fn unpair(self) -> Result<(Term, Term), Error> {
		if let Abs(_) = self {
			if let Ok((wrapped_a, b)) = self.unabs().and_then(|t| t.unapp()) {
				Ok((try!(wrapped_a.rhs()), b))
			} else {
				Err(NotAPair)
			}
		} else {
			if let Ok((wrapped_a, b)) = self.unapp() {
				Ok((try!(wrapped_a.rhs()), b))
			} else {
				Err(NotAPair)
			}
		}
	}

	pub fn fst(self) -> Result<Term, Error> {
		if let Abs(_) = self {
			self.unabs()
				.and_then(|t| t.lhs())
				.and_then(|t| t.rhs())
		} else {
			self.lhs().and_then(|t| t.rhs())
		}
	}

	pub fn fst_ref(&self) -> Result<&Term, Error> {
		if let Abs(_) = *self {
			self.unabs_ref()
				.and_then(|t| t.lhs_ref())
				.and_then(|t| t.rhs_ref())
		} else {
			self.lhs_ref().and_then(|t| t.rhs_ref())
		}
	}

	pub fn fst_ref_mut(&mut self) -> Result<&mut Term, Error> {
		if let Abs(_) = *self {
			self.unabs_ref_mut()
				.and_then(|t| t.lhs_ref_mut())
				.and_then(|t| t.rhs_ref_mut())
		} else {
			self.lhs_ref_mut().and_then(|t| t.rhs_ref_mut())
		}
	}

	pub fn snd(self) -> Result<Term, Error> {
		if let Abs(_) = self {
			self.unabs().and_then(|t| t.rhs())
		} else {
			self.rhs()
		}
	}

	pub fn snd_ref(&self) -> Result<&Term, Error> {
		if let Abs(_) = *self {
			self.unabs_ref().and_then(|t| t.rhs_ref())
		} else {
			self.rhs_ref()
		}
	}

	pub fn snd_ref_mut(&mut self) -> Result<&mut Term, Error> {
		if let Abs(_) = *self {
			self.unabs_ref_mut().and_then(|t| t.rhs_ref_mut())
		} else {
			self.rhs_ref_mut()
		}
	}

	pub fn is_list(&self) -> bool {
		self.is_empty() || self.head_ref().is_ok() && self.tail_ref().is_ok()
	}

	pub fn is_empty(&self) -> bool {
		self.head_ref() == Ok(&nil())
	}

	pub fn uncons(self) -> Result<(Term, Term), Error> {
		self.unabs()
			.and_then(|t| t.snd())
			.and_then(|t| t.unpair())
	}

	pub fn head(self) -> Result<Term, Error> {
		self.unabs()
			.and_then(|t| t.snd())
			.and_then(|t| t.fst())
	}

	pub fn head_ref(&self) -> Result<&Term, Error> {
		self.unabs_ref()
			.and_then(|t| t.snd_ref())
			.and_then(|t| t.fst_ref())
	}

	pub fn head_ref_mut(&mut self) -> Result<&mut Term, Error> {
		self.unabs_ref_mut()
			.and_then(|t| t.snd_ref_mut())
			.and_then(|t| t.fst_ref_mut())
	}

	pub fn tail(self) -> Result<Term, Error> {
		self.unabs()
			.and_then(|t| t.snd())
			.and_then(|t| t.snd())
	}

	pub fn tail_ref(&self) -> Result<&Term, Error> {
		self.unabs_ref()
			.and_then(|t| t.snd_ref())
			.and_then(|t| t.snd_ref())
	}

	pub fn tail_ref_mut(&mut self) -> Result<&mut Term, Error> {
		self.unabs_ref_mut()
			.and_then(|t| t.snd_ref_mut())
			.and_then(|t| t.snd_ref_mut())
	}

	pub fn len(&self) -> Result<usize, Error> {
		unimplemented!()
	}
}

/*
pub fn from(elems: Vec<Term>) -> List {
	let mut list = List::new();

	for elem in elems.into_iter().rev() { list.push(elem); }

	list
}

pub fn len(&self) -> usize { // TODO: maybe no cloning?
	let mut copy = (*self).clone();
	let mut n = 0;

	while let Some(_) = copy.pop() { n += 1; }

	n
}

pub fn push(&mut self, term: Term) { // TODO: maybe no cloning?
	let tail = self.0.clone();
	*self = List(abs(app(Var(0), app(term, tail))))
}
*/
/*
impl Iterator for Term {
	type Item = Term;

	fn next(&mut self) -> Option<Term> {
		if self.is_empty() {
			None
		} else {
			self.pop()
		}
	}
}
*/
#[cfg(test)]
mod test {
	use super::*;
	use arithmetic::*;
	use reduction::*;

	#[test]
	fn pair_operations() {
		let pair_four_three = normalize(pair().app(to_cnum(4)).app(to_cnum(3)));

		assert!(pair_four_three.is_pair());

		assert_eq!(pair_four_three.fst_ref(), Ok(&to_cnum(4)));
		assert_eq!(pair_four_three.snd_ref(), Ok(&to_cnum(3)));

		let unpaired = pair_four_three.unpair();
		assert_eq!(unpaired, Ok((to_cnum(4), to_cnum(3))));
	}

	#[test]
	fn list_operations() {
		let list_three_five_four = normalize(cons().app(to_cnum(3)).app(cons().app(to_cnum(5)).app(cons().app(to_cnum(4)).app(nil()))));

		assert!(list_three_five_four.is_list());

		assert_eq!(list_three_five_four.head_ref(), Ok(&to_cnum(3)));
		assert_eq!(list_three_five_four.tail_ref(), Ok(&normalize(cons().app(to_cnum(5)).app(cons().app(to_cnum(4)).app(nil())))));

		assert_eq!(list_three_five_four.tail_ref().and_then(|t| t.head_ref()), Ok(&to_cnum(5)));
		assert_eq!(list_three_five_four.tail_ref().and_then(|t| t.head_ref()), Ok(&to_cnum(5)));
		assert_eq!(list_three_five_four.tail_ref().and_then(|t| t.tail_ref()).and_then(|t| t.head_ref()), Ok(&to_cnum(4)));

		let unconsed = list_three_five_four.uncons();
		assert_eq!(unconsed, Ok((to_cnum(3), normalize(cons().app(to_cnum(5)).app(cons().app(to_cnum(4)).app(nil()))))));
	}

/*
	#[test]
	fn empty_list() {
		let mut list = List::new();

		assert!(list.is_empty());
		list.push(Var(1));
		assert!(!list.is_empty());
	}

	#[test]
	fn list_push_pop() {
		let mut list = List::from(vec![Var(1), Var(0), Var(1)]);

		assert_eq!(list.pop(), Some(Var(1)));
		assert_eq!(list.pop(), Some(Var(0)));
		assert_eq!(list.pop(), Some(Var(1)));
		assert_eq!(list.pop(), None);
	}

	#[test]
	fn list_len() {
		let mut list = List::new();

		assert_eq!(list.len(), 0);
		list.push(Var(0));
		assert_eq!(list.len(), 1);
		list.push(Var(0));
		assert_eq!(list.len(), 2);
	}

	#[test]
	fn iterating_list() {
		let list = List::from(vec![Var(0), Var(1), Var(0)]);
		let mut iter = list.into_iter();

		assert_eq!(iter.next(), Some(Var(0)));
		assert_eq!(iter.next(), Some(Var(1)));
		assert_eq!(iter.next(), Some(Var(0)));
	}
*/
}