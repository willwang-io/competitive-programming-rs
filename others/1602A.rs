// Created: Aug 18 2026, 23:57:41
// Formatted with rustfmt.

fn solve() {
    let s = read::<String>().into_bytes();
    let mn = *s.iter().min().unwrap();
    let i = s.iter().position(|&b| b == mn).unwrap();
    println!(
        "{} {}{}",
        mn as char,
        String::from_utf8(s[..i].to_vec()).unwrap(),
        String::from_utf8(s[i + 1..].to_vec()).unwrap()
    );
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
