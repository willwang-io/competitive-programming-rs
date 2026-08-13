// Created: Aug 13 2026, 02:06:20
// Formatted with rustfmt.

fn solve() {
    let mut a = read::<String>().into_bytes();
    let mut b = read::<String>().into_bytes();

    (a[0], b[0]) = (b[0], a[0]);
    println!(
        "{} {}",
        String::from_utf8(a).unwrap(),
        String::from_utf8(b).unwrap()
    );
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
