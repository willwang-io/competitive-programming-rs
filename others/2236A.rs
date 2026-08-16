// Created: Aug 16 2026, 08:54:08
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let a: Vec<i32> = (0..n).map(|_| read()).collect();

    let mx = a.iter().max().unwrap();
    let mn = a.iter().min().unwrap();

    println!("{}", mx + 1 - mn);
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

