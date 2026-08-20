// Created: Aug 19 2026, 21:37:01
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();

    if n % 2 == 0 {
        println!("-1");
        return;
    }

    let a: Vec<String> = (0..n)
        .map(|i| if i % 2 == 0 { n - i / 2 } else { (i + 1) / 2 }.to_string())
        .collect();

    let ans = a.join(" ");
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
