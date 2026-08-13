// Created: Aug 12 2026, 18:29:37
// Formatted with rustfmt.

fn main() {
    let n: usize = read();
    let x: i32 = read();
    let a: i32 = (0..n).map(|_| read::<i32>()).sum::<i32>().abs();
    println!("{}", (a + x - 1) / x);
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
