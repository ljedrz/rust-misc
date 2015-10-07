// A palindromic number reads the same both ways.
// The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 x 99.
// Find the largest palindrome made from the product of two 3-digit numbers.

fn is_palindrome<T: ToString>(value: T) -> bool {
    let str1 = value.to_string();
    let str2 = str1.chars().rev().collect::<String>();

    str1 == str2
}

fn main() {
    let (mut candidate, mut max) = (1u64, 1u64);

    for i in 1..1000 {
        for j in 1 ..1000 {
            candidate = i * j;
            if is_palindrome(candidate) && candidate > max { max = candidate; }
        }
    }

    assert_eq!(max, 906609);
    println!("{}", max);
}