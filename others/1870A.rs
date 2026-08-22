// Created: Aug 21 2026, 09:22:51
// Formatted with rustfmt.

fn solve() {
    let n: i32 = read();
    let k: i32 = read();
    let x: i32 = read();
    if k > n || k - 1 > x {
        println!("-1");
        return;
    }

    let v = if k == x { k - 1 } else { x };
    let ans = k * (k - 1) / 2 + (n - k) * v;
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
