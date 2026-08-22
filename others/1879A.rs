// Created: Aug 22 2026, 11:34:39
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let s: i64 = read();
    let e: i64 = read();
    let mut ok = true;

    for _ in 1..n {
        let x: i64 = read();
        let y: i64 = read();

        if x >= s && y >= e {
            ok = false;
        }
    }

    if ok {
        println!("{s}");
    } else {
        println!("-1");
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
