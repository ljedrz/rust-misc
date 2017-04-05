use term::*;
use term::Term::*;
use term::Error::*;
use booleans::*;
use reduction::*;

// PAIR := λxyf.f x y
pub fn pair() -> Term { abs(abs(abs(Var(1).app(Var(3)).app(Var(2))))) }

// FIRST := λp.p TRUE
pub fn first() -> Term { abs(Var(1).app(tru())) }

// SECOND := λp.p FALSE
pub fn second() -> Term { abs(Var(1).app(fls())) }

// NIL := λx.TRUE
pub fn nil() -> Term { abs(tru()) }

// NULL := λp.p (λxy.FALSE)
pub fn null() -> Term { abs(Var(1).app(abs(abs(fls())))) }

// CONS := λht.PAIR FALSE (PAIR h t)
pub fn cons() -> Term { abs(abs(pair().app(fls()).app(pair().app(Var(2)).app(Var(1))))) }

// HEAD := λz.FIRST (SECOND z)
pub fn head() -> Term { abs(first().app(second().app(Var(1)))) }

// TAIL := λz.SECOND (SECOND z)
pub fn tail() -> Term { abs(second().app(second().app(Var(1)))) }

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
		*self == nil()
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
		let mut inner = self;
		let mut n = 0;

		while *inner != nil() {
			n += 1;
			inner = try!(inner.tail_ref());
		}

		Ok(n)
	}

	pub fn push(self, t: Term) -> Term {
		normalize(cons().app(t).app(self))
	}
}

impl From<Vec<Term>> for Term {
	fn from(terms: Vec<Term>) -> Self {
		let mut output = nil();

		for t in terms {
			output = cons().app(t).app(output);
		}

		normalize(output)
	}
}
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

	#[test]
	fn empty_list() {
		let empty_list = nil();

		assert!(empty_list.is_list());
		assert!(empty_list.is_empty());
	}

	#[test]
	fn list_length() {
		let list0 = nil();
		assert_eq!(list0.len(), Ok(0));
		let list1 = normalize(cons().app(one()).app(list0));
		assert_eq!(list1.len(), Ok(1));
		let list2 = normalize(cons().app(one()).app(list1));
		assert_eq!(list2.len(), Ok(2));
		let list3 = normalize(cons().app(one()).app(list2));
		assert_eq!(list3.len(), Ok(3));
	}

	#[test]
	fn list_from_vector() {
		let terms_vec = vec![one(), zero(), one()];
		let list_from_vec = Term::from(terms_vec);
		let list_manual = normalize(cons().app(one()).app(cons().app(zero()).app(cons().app(one()).app(nil()))));

		assert_eq!(list_from_vec, list_manual);
	}

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
		let empty_list = nil();
		let list_four = cons().app(to_cnum(4)).app(empty_list);
		let list_five_four = cons().app(to_cnum(5)).app(list_four);
		let list_three_five_four = normalize(cons().app(to_cnum(3)).app(list_five_four));

		assert!(list_three_five_four.is_list());

		assert_eq!(list_three_five_four.head_ref(), Ok(&to_cnum(3)));
		assert_eq!(list_three_five_four.tail_ref(), Ok(&normalize(cons().app(to_cnum(5)).app(cons().app(to_cnum(4)).app(nil())))));

		assert_eq!(list_three_five_four.tail_ref().and_then(|t| t.head_ref()), Ok(&to_cnum(5)));
		assert_eq!(list_three_five_four.tail_ref().and_then(|t| t.head_ref()), Ok(&to_cnum(5)));
		assert_eq!(list_three_five_four.tail_ref().and_then(|t| t.tail_ref()).and_then(|t| t.head_ref()), Ok(&to_cnum(4)));

		let unconsed = list_three_five_four.uncons();
		assert_eq!(unconsed, Ok((to_cnum(3), normalize(cons().app(to_cnum(5)).app(cons().app(to_cnum(4)).app(nil()))))));
	}

	#[test]
	fn list_push() {
		let list_pushed = nil().push(one()).push(zero()).push(one());
		let list_manual = normalize(cons().app(one()).app(cons().app(zero()).app(cons().app(one()).app(nil()))));

		assert_eq!(list_pushed, list_manual);
	}

/*
	#[test]
	fn list_pop() {
		let mut list = List::from(vec![Var(1), Var(0), Var(1)]);

		assert_eq!(list.pop(), Some(Var(1)));
		assert_eq!(list.pop(), Some(Var(0)));
		assert_eq!(list.pop(), Some(Var(1)));
		assert_eq!(list.pop(), None);
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