// Created: Aug 13 2026, 01:37:46
// Formatted with rustfmt.

use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let last = s
        .chars()
        .rev()
        .find(|c| c.is_ascii_alphabetic())
        .unwrap()
        .to_ascii_lowercase();

    let ans = if matches!(last, 'a' | 'e' | 'i' | 'o' | 'u' | 'y') {
        "YES"
    } else {
        "NO"
    };

    println!("{ans}");
}

thread_local! {
    pub static INPUT: std::cell::RefCell<std::str::SplitAsciiWhitespace<'static>> = std::cell::RefCell::<std::str::SplitAsciiWhitespace<'static>>::new({
        let mut input = String::new();
        std::io::Read::read_to_string(&mut std::io::stdin(), &mut input).unwrap();
        Box::leak(input.into_boxed_str()).split_ascii_whitespace()
    });
}

pub fn read<T: std::str::FromStr>() -> T
where
    T::Err: std::fmt::Debug,
{
    INPUT.with(|input| input.borrow_mut().next().unwrap().parse().unwrap())
}
