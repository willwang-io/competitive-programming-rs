// Created: Aug  9 2026, 20:22:33
// Formatted with rustfmt.

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b > 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn solve() {
    let mut n: i64 = read();

    loop {
        let dsum: i64 = n.to_string().bytes().map(|b| (b - b'0') as i64).sum();
        if gcd(n, dsum) > 1 {
            println!("{n}");
            return;
        }
        n += 1;
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
