// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we see that the 6th prime is 13.
// What is the 10001st prime number?

fn factors(number: u64) -> Vec<u64> {
    let mut n = 1;
    let mut factors = Vec::new();

    while n * n <= number {
        if number % n == 0 { factors.push(n) }
        n += 1;
    }

    factors.push(number);

    factors
}

fn is_prime(number: u64) -> bool {
    factors(number).len() == 2
}

fn main() {
	let mut count = 0;

	for n in (2u64..) {
		if is_prime(n) {
			count += 1;
			if count == 10001 {
                assert_eq!(n, 104743);
				println!("{}", n);
				return;
			}
		}
	}
}