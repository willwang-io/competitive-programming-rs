// Created: Aug 12 2026, 22:59:59
// Formatted with rustfmt.

fn main() {
    let s1: String = read();
    let s2: String = read();

    let ans: String = s1
        .bytes()
        .zip(s2.bytes())
        .map(|(x, y)| char::from(((x - b'0') ^ (y - b'0')) + b'0'))
        .collect();

    println!("{ans}");
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
