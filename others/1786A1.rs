// Created: Aug  3 2026, 01:16:54
// Formatted with rustfmt.

fn solve() {
    let mut n: i32 = read();
    let mut a = 0;
    let mut b = 0;
    let mut i = 1;
    while n > 0 {
        let x = n.min(i);
        if i % 4 == 0 || i % 4 == 1 {
            a += x;
        } else {
            b += x;
        }
        n -= x;
        i += 1;
    }
    println!("{a} {b}");
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
