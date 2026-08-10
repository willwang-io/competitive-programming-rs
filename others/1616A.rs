// Created: Aug  9 2026, 15:57:24
// Formatted with rustfmt.

use std::collections::HashMap;

fn solve() {
    let n: usize = read();

    let mut cnt: HashMap<i32, i32> = HashMap::new();

    for _ in 0..n {
        let x: i32 = read();
        *cnt.entry(x).or_default() += 1;
    }

    for (k, v) in cnt {}
}

fn main() {
    let t: usize = read();
    for _ in 0..t {
        solve();
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
