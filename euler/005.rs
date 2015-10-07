// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without
// any remainder.
// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

fn is_divisible(number: u64) -> bool {
    for n in (2..21) {
        if number % n != 0 { return false; }
    }
    true
}

fn main() {
    for n in (1u64..) {
        if is_divisible(n) {
            assert_eq!(n, 232792560);
            println!("{}", n);
            return;
        }
    }
}