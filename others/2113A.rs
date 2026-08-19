// Created: Aug 18 2026, 19:47:24
// Formatted with rustfmt.

fn solve() {
    let mut k: i64 = read();
    let mut a: i64 = read();
    let mut b: i64 = read();
    let mut x: i64 = read();
    let mut y: i64 = read();

    if x > y {
        (a, b) = (b, a);
        (x, y) = (y, x);
    }

    let mut ans = 0;

    if k >= a {
        let cnt = (k - a) / x + 1;
        ans += cnt;
        k -= cnt * x;
    }

    if k >= b {
        ans += (k - b) / y + 1;
    }

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
