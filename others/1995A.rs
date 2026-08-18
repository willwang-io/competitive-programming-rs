// Created: Aug 17 2026, 17:16:46
// Formatted with rustfmt.

fn solve() {
    let n: i32 = read();
    let mut k: i32 = read();
    let mut ans = 0;
    for x in (1..=n).rev() {
        for _ in 0..if x == n { 1 } else { 2 } {
            if k > 0 {
                k -= x;
                ans += 1;
            }
        }
    }
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
