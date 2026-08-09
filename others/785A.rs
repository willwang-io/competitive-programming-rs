// Created: Aug  9 2026, 11:46:30
// Formatted with rustfmt.

fn main() {
    let n: usize = read();
    let a: Vec<String> = (0..n).map(|_| read()).collect();
    let ans: i32 = a
        .iter()
        .map(|s| {
            if s.starts_with("T") {
                4
            } else if s.starts_with("C") {
                6
            } else if s.starts_with("O") {
                8
            } else if s.starts_with("D") {
                12
            } else {
                20
            }
        })
        .sum();
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
