// Created: Aug  9 2026, 11:50:47
// Formatted with rustfmt.

fn solve() {
    let x: i32 = read();
    let mut ans = 0;
    for i in 1..10 {
        let mut cur = 0;
        for j in 1..5 {
            cur = cur * 10 + i;
            ans += j;
            if cur == x {
                println!("{ans}");
                return;
            }
        }
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
