// Created: Aug 13 2026, 10:16:08
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let k: usize = read();
    let s = read::<String>().chars().collect::<Vec<_>>();

    let mut ans = 0;
    let mut i = 0;
    while i < n {
        if s[i] == '0' {
            let mut j = i;
            while j < n && s[j] == '0' && j - i <= k {
                j += 1;
            }
            if j - i > k || j == n {
                ans += 1;
                i += k;
            } else {
                i = j - 1;
            }
        } else {
            i += k;
        }
        i += 1;
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
