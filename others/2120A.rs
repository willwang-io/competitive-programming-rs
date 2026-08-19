// Created: Aug 18 2026, 19:34:13
// Formatted with rustfmt.

fn solve() {
    let l1: i32 = read();
    let b1: i32 = read();
    let l2: i32 = read();
    let b2: i32 = read();
    let l3: i32 = read();
    let b3: i32 = read();

    let ok = l1 == l2 && l2 == l3 && b1 + b2 + b3 == l1
        || b1 == b2 && b2 == b3 && l1 + l2 + l3 == b1
        || l2 == l3 && b2 + b3 == b1 && l1 + l2 == b1
        || b2 == b3 && l2 + l3 == l1 && b1 + b2 == l1;

    let ans = if ok { "YES" } else { "NO" };
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
