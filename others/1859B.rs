// Created: Aug 22 2026, 12:19:45
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let mut first = i64::MAX;
    let mut second = i64::MAX;
    let mut sum = 0;

    for _ in 0..n {
        let m: usize = read();
        let mut a = i64::MAX;
        let mut b = i64::MAX;

        for _ in 0..m {
            let x: i64 = read();

            if x < a {
                b = a;
                a = x;
            } else if x < b {
                b = x;
            }
        }

        first = first.min(a);
        second = second.min(b);
        sum += b;
    }

    let ans = first + sum - second;
    println!("{ans}");
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
