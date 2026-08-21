// Created: Aug  3 2026, 01:11:36
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let mut prev: i32 = read();
    let mut ok = true;
    for i in 1..n {
        let x: i32 = read();
        if prev > x && !i.is_power_of_two() {
            ok = false;
        }
        prev = x;
    }
    if ok {
        println!("YES");
    } else {
        println!("NO");
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
