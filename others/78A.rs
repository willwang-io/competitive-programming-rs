// Created: Aug 13 2026, 01:57:57
// Formatted with rustfmt.

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let a = [5, 7, 5];
    let mut ok = true;
    for (i, line) in stdin.lock().lines().enumerate() {
        let line = line.unwrap();
        let cnt = line
            .chars()
            .filter(|&c| c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u')
            .count();
        if cnt != a[i] {
            ok = false;
        }
    }

    if ok {
        println!("YES");
    } else {
        println!("NO");
    }
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
