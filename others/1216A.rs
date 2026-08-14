// Created: Aug 13 2026, 19:04:45
// Formatted with rustfmt.

fn main() {
    let n: usize = read();
    let mut s = read::<String>().into_bytes();
    let mut cnt = 0;
    for i in (0..n).step_by(2) {
        if s[i] == s[i + 1] {
            s[i] = if s[i] == b'a' { b'b' } else { b'a' };
            cnt += 1;
        }
    }
    let ans = String::from_utf8(s).unwrap();
    println!("{cnt}\n{ans}");
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
