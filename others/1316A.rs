// Created: Aug 11 2026, 14:13:48
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let m: i64 = read();
    let a: Vec<i64> = (0..n).map(|_| read()).collect();

    let mut ans = a[0];
    for i in 1..n {
        let diff = m - ans;
        ans += diff.min(a[i]);
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
