// Created: Aug 10 2026, 22:59:17
// Formatted with rustfmt.

fn main() {
    let n: usize = read();
    let mut a: Vec<i32> = (0..n).map(|_| read()).collect();
    let m: usize = read();
    let mut b: Vec<i32> = (0..m).map(|_| read()).collect();
    a.sort_unstable();
    b.sort_unstable();
    println!("{} {}", a[n - 1], b[m - 1]);
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
