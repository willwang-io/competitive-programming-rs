// Created: Aug 19 2026, 21:51:02
// Formatted with rustfmt.

fn solve() {
    let k: usize = read();
    let q: usize = read();
    let a: Vec<usize> = (0..k).map(|_| read()).collect();

    let ans = (0..q)
        .map(|_| read::<usize>().min(a[0] - 1).to_string())
        .collect::<Vec<_>>()
        .join(" ");

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
