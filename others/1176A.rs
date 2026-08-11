// Created: Aug 11 2026, 00:47:40
// Formatted with rustfmt.

fn solve() {
    let mut n: i64 = read();
    let mut ans = 0;
    while n % 5 == 0 {
        n = 4 * n / 5;
        ans += 1;
    }
    while n % 3 == 0 {
        n = 2 * n / 3;
        ans += 1;
    }
    while n % 2 == 0 {
        n /= 2;
        ans += 1;
    }
    if n != 1 {
        println!("-1");
    } else {
        println!("{ans}");
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
