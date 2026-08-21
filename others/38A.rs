// Created: Aug  2 2026, 01:37:29
// Formatted with rustfmt.

fn main() {
    let n: usize = read();
    let d: Vec<i32> = (1..n).map(|_| read()).collect();
    let a: usize = read();
    let b: usize = read();
    let ans = d[a - 1..b - 1].iter().sum::<i32>();
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
