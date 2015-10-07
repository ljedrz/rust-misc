// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143 ?

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
    let result = factors(600851475143).iter().filter(|&f| is_prime(*f)).last().unwrap().clone();

    assert_eq!(result, 6857);
    println!("{}", result);
}