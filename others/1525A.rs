// Created: Aug 10 2026, 10:28:29
// Formatted with rustfmt.

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b > 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn solve() {
    let k: i32 = read();
    println!("{}", 100 / gcd(100, k));
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
