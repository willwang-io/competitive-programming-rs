// Created: Aug 21 2026, 09:30:23
// Formatted with rustfmt.

fn solve() {
    let x: i32 = read();
    let y: i32 = read();
    let n: usize = read();

    if y - x < (n * (n - 1) / 2) as i32 {
        println!("-1");
        return;
    }

    let mut a = vec![x; n];
    a[n - 1] = y;

    for i in (1..n - 1).rev() {
        a[i] = a[i + 1] - (n - i - 1) as i32;
    }

    let ans = a
        .iter()
        .map(|&x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ");

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
