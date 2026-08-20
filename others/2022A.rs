// Created: Aug 19 2026, 20:10:55
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let mut r: i32 = read();
    let mut sum = 0;
    let mut odd = 0;

    for _ in 0..n {
        let a: i32 = read();
        sum += a;
        odd += a % 2;
        r -= a / 2;
    }

    let ans = sum - 2 * (odd - r).max(0);
    println!("{ans}");
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
