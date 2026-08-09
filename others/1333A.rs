// Created: Aug  9 2026, 12:07:24
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let m: usize = read();
    let ans = (0..n)
        .map(|i| {
            (0..m)
                .map(|j| if i == 0 && j == 0 { 'W' } else { 'B' })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n");
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

