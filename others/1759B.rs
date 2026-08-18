// Created: Aug 18 2026, 09:54:25
// Formatted with rustfmt.

fn solve() {
    let m: usize = read();
    let s: i32 = read();
    let a: Vec<i32> = (0..m).map(|_| read()).collect();

    let sum = a.iter().sum::<i32>() + s;
    let mx = *a.iter().max().unwrap();

    let mut n = 1;
    while n * (n + 1) / 2 < sum {
        n += 1;
    }

    if n >= mx && n * (n + 1) / 2 == sum {
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
