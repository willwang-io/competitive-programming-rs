// Created: Aug  8 2026, 12:40:16
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let k: usize = read();
    let s: Vec<char> = read::<String>().chars().collect();

    if k * 2 > n {
        println!("-1");
        return;
    }
    let mut ans = 0;
    for i in 0..k {
        if s[i] == 'L' {
            ans += 1;
        }
        if s[n - i - 1] == 'R' {
            ans += 1;
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
