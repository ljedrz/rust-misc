// The sum of the squares of the first ten natural numbers is 385
// The square of the sum of the first ten natural numbers is 3025
// Hence the difference between the sum of the squares of the first ten natural numbers and the
// square of the sum is 3025 − 385 = 2640.
// Find the difference between the sum of the squares of the first one hundred natural numbers and
// the square of the sum.

fn main() {
    let result = (1u64..101).fold(0, |sum, acc| sum + acc).pow(2) -
        (1u64..101).map(|n| n.pow(2)).fold(0, |sum, acc| sum + acc);

    assert_eq!(result, 25164150);
    println!("{}", result);
}