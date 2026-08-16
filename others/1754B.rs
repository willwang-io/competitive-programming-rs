// Created: Aug 15 2026, 16:35:35
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let m = n / 2;
    let mut ans = vec![];
    for i in 1..=m {
        ans.push(i + m);
        ans.push(i);
    }

    if n % 2 == 1 {
        ans.push(n);
    }

    let ans = ans
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
