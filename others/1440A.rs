// Created: Aug 11 2026, 18:47:55
// Formatted with rustfmt.

fn solve() {
    let _: i32 = read();
    let c0: i32 = read();
    let c1: i32 = read();
    let h: i32 = read();
    let s: String = read();

    let ans: i32 = s
        .bytes()
        .map(|c| {
            if c == b'0' {
                c0.min(c1 + h)
            } else {
                c1.min(c0 + h)
            }
        })
        .sum();

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
