// Created: Aug 16 2026, 09:26:38
// Formatted with rustfmt.

fn is_prime(x: i64) -> bool {
    if x < 2 {
        return false;
    }

    let mut i = 2;
    while i <= x / i {
        if x % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

fn next_prime(mut x: i64) -> i64 {
    while !is_prime(x) {
        x += 1;
    }
    x
}

fn solve() {
    let d: i64 = read();
    let p = next_prime(d + 1);
    let q = next_prime(p + d);
    let ans = p * q;
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
