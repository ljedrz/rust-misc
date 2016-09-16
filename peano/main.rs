trait Peano {}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Zero;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Succ<T: Peano>(T);

impl Peano for Zero {}
impl<T> Peano for Succ<T> where T: Peano {}

trait Next: Peano {
    type Next: Peano;
}

impl Next for Zero {
    type Next = Succ<Zero>;
}

impl<T> Next for Succ<T> where T: Peano {
    type Next = Succ<Succ<T>>;
}

trait Prev: Peano {
    type Prev: Peano;
}

impl<T> Prev for Succ<T> where T: Peano {
    type Prev = T;
}

trait Value: Peano {
    fn value() -> usize;
}

impl Value for Zero {
    fn value() -> usize { 0 }
}

impl<T> Value for Succ<T> where T: Value {
    fn value() -> usize { T::value() + 1 }
}

fn main() {
	assert_eq!(Zero::value(), 0);
	assert_eq!(<Zero as Next>::Next::value(), 1);
	assert_eq!(<Succ<Zero> as Next>::Next::value(), 2);
	assert_eq!(<Succ<Succ<Zero>> as Next>::Next::value(), 3);

	assert_eq!(<Succ<Zero> as Prev>::Prev::value(), 0);
	assert_eq!(<Succ<Succ<Zero>> as Prev>::Prev::value(), 1);
}
