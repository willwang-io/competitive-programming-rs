// Created: Aug 12 2026, 18:52:36
// Formatted with rustfmt.

fn main() {
    let x1: i32 = read();
    let y1: i32 = read();
    let x2: i32 = read();
    let y2: i32 = read();

    println!("{}", (x1 - x2).abs().max((y1 - y2).abs()));
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
