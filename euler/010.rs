// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
// Find the sum of all the primes below two million.

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
    let mut sum = 0u64;

    for n in (2..2000000) {
        if is_prime(n) { sum += n; }
    }

    assert_eq!(sum, 142913828922);
    println!("{}", sum);
}