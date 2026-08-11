// Created: Aug 11 2026, 00:53:16
// Formatted with rustfmt.

fn solve() {
    let s: String = read();
    let mut s: Vec<_> = s
        .split("0")
        .filter(|&x| !x.is_empty())
        .map(|x| x.len())
        .collect();
    s.sort_unstable();
    s.reverse();

    let ans: usize = s.iter().step_by(2).sum();
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
