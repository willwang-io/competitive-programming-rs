// Created: Aug 11 2026, 14:47:59
// Formatted with rustfmt.

fn main() {
    let n: i32 = read();
    for a in (1..=n.isqrt()).rev() {
        if n % a == 0 {
            println!("{a} {}", n / a);
            return;
        }
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
