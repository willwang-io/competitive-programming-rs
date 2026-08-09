// Created: Aug  9 2026, 08:34:54
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let k: i64 = read();
    let s = read::<String>().into_bytes();
    let mut red = 0;
    let mut total = 0;

    for i in 0..2 * n {
        if s[i] == b'1' {
            total += 1;
            if s[(i + 1) % (2 * n)] == if i % 2 == 0 { b'0' } else { b'1' } {
                red += 1;
            }
        }
    }

    println!("{} {}", red, total - red);
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
