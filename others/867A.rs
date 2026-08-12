// Created: Aug 11 2026, 21:11:20
// Formatted with rustfmt.

fn main() {
    let n: usize = read();
    let s = read::<String>().into_bytes();
    if s[0] == b'S' && s[n - 1] == b'F' {
        println!("YES");
    } else {
        println!("NO");
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
