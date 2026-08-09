// Created: Aug  9 2026, 11:55:02
// Formatted with rustfmt.

fn solve() {
    let n: i64 = read();
    if n % 3 == 0 {
        println!("{} {}", n / 3, n / 3);
    } else if n % 3 == 1 {
        println!("{} {}", n / 3 + 1, n / 3);
    } else {
        println!("{} {}", n / 3, n / 3 + 1);
    }
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
