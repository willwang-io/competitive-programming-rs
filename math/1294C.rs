// Created: Aug 13 2026, 10:39:25
// Formatted with rustfmt.

fn solve() {
    let mut n: i64 = read();
    let mut a = 0;
    let mut b = 0;
    for i in 2..n.isqrt() + 1 {
        if n % i == 0 {
            if a == 0 {
                a = i;
            } else {
                b = i;
            }
            n /= i;
        }
        if a != 0 && b != 0 {
            break;
        }
    }

    if a == 0 || b == 0 || a == n || b == n {
        println!("NO");
    } else {
        println!("YES\n{a} {b} {n}");
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
