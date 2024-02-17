use core::fmt::Display;
use num_traits::PrimInt;
use std::iter::zip;

fn main() {
    for (n, o) in (0..39)
        .map(fibonacci)
        .filter(no_consecutive_digits)
        .enumerate()
    {
        println!("{} : {}", n, o);
    }
}

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 2) + fibonacci(n - 1),
    }
}

trait Pairable {
    fn adjacent_digit_pairs(self) -> Vec<(u8, u8)>
    where
        Self: Sized;
    fn digits(self) -> Vec<u8>
    where
        Self: Sized;
}

impl<T: PrimInt + Display> Pairable for T {
    fn adjacent_digit_pairs(self) -> Vec<(u8, u8)> {
        zip(self.digits(), self.digits().drain(1..)).collect()
    }

    fn digits(self) -> Vec<u8> {
        self.to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect()
    }
}

fn no_consecutive_digits(n: &u64) -> bool {
    n.adjacent_digit_pairs().iter().filter(consecutive).count() == 0
}

fn consecutive((a, b): &&(u8, u8)) -> bool {
    return a.abs_diff(*b) == 1;
}
