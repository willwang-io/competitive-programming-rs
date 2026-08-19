// Created: Aug 19 2026, 17:24:28
// Formatted with rustfmt.

fn solve() {
    let mut s = read::<String>().into_bytes();

    s[1] = if s[0] < s[2] {
        b'<'
    } else if s[0] > s[2] {
        b'>'
    } else {
        b'='
    };

    let ans = String::from_utf8(s).unwrap();
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
