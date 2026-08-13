// Created: Aug 12 2026, 21:19:19
// Formatted with rustfmt.

fn main() {
    let mut a: i32 = read();
    let mut b: i32 = read();
    let x = a.isqrt();
    if x * x + x <= b {
        println!("Vladik");
    } else {
        println!("Valera");
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
