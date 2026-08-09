// Created: Aug  9 2026, 11:53:19
// Formatted with rustfmt.

fn solve() {
    let mut a: Vec<i32> = (0..4).map(|_| read()).collect();
    a.sort_unstable();
    println!("{} {} {}", a[3] - a[0], a[3] - a[1], a[3] - a[2]);
}

fn main() {
    solve();
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
