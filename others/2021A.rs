// Created: Aug 19 2026, 20:26:56
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let mut a: Vec<i64> = (0..n).map(|_| read()).collect();
    a.sort_unstable();

    let mut ans = a[0];
    for &x in &a[1..] {
        ans = (ans + x) / 2;
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
