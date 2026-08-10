// Created: Aug 10 2026, 10:17:54
// Formatted with rustfmt.

fn solve() {
    let s = read::<String>().into_bytes();
    let mut ans = vec![];
    for i in 0..s.len() {
        if i % 2 == 0 {
            if s[i] == b'a' {
                ans.push(b'b');
            } else {
                ans.push(b'a');
            }
        } else {
            if s[i] == b'z' {
                ans.push(b'y');
            } else {
                ans.push(b'z');
            }
        }
    }
    println!("{}", String::from_utf8(ans).unwrap());
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
