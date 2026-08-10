// Created: Aug 10 2026, 10:06:48
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let a: Vec<i32> = (0..n).map(|_| read()).collect();
    let even = a
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    let odd = a
        .iter()
        .filter(|&&x| x % 2 != 0)
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    if even.is_empty() {
        println!("{odd}");
    } else {
        println!("{even} {odd}");
    }
}

fn main() {
    let t: usize = read();
    for _ in 0..t {
        solve();
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
