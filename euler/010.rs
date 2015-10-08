// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
// Find the sum of all the primes below two million.

fn is_prime(number: u64) -> bool {
    let mut n = 2;

    while n * n <= number {
        if number % n == 0 { return false; }
        n += 1;
    }

    true
}

fn main() {
    let mut sum = 0u64;

    for n in (2..2000000) { if is_prime(n) { sum += n; } }

    assert_eq!(sum, 142913828922);
    println!("{}", sum);
}