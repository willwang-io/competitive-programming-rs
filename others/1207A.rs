// Created: Aug 11 2026, 19:10:21
// Formatted with rustfmt.

fn solve() {
    let b = read::<i32>() / 2;
    let mut p: i32 = read();
    let mut f: i32 = read();
    let mut h: i32 = read();
    let mut c: i32 = read();

    if h < c {
        (p, f) = (f, p);
        (h, c) = (c, h);
    }

    let x = b.min(p);
    let y = (b - x).min(f);
    let ans = x * h + y * c;
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
