// Created: Aug  8 2026, 12:36:06
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let mut l = 0;
    let mut r = i64::MAX / 4;
    for i in 1..=n {
        let w: i64 = read();
        if i % 2 == 0 {
            l = l.max(w);
        } else {
            r = r.min(w);
        }
    }
    if n % 2 == 0 && l + 2 <= r {
        println!("YES");
    } else {
        println!("NO");
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
