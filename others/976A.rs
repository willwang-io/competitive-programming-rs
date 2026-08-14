// Created: Aug 13 2026, 19:19:46
// Formatted with rustfmt.

fn main() {
    let n: usize = read();
    let s = read::<String>().into_bytes();

    let zero = s.iter().filter(|&&c| c == b'0').count();
    let one = n - zero;
    if one == 0 {
        println!("{}", "0".repeat(zero));
    } else {
        println!("1{}", "0".repeat(zero));
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
