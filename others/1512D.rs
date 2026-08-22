// Created: Aug 22 2026, 12:09:14
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let mut b: Vec<i64> = (0..n + 2).map(|_| read()).collect();
    b.sort_unstable();

    let sum: i64 = b[..n].iter().sum();
    let a = if sum == b[n] {
        b[..n].to_vec()
    } else {
        let sum = sum + b[n];
        let x = sum - b[n + 1];

        if let Some(i) = b[..=n].iter().position(|&v| v == x) {
            b[..=n]
                .iter()
                .enumerate()
                .filter(|&(j, _)| j != i)
                .map(|(_, &v)| v)
                .collect()
        } else {
            println!("-1");
            return;
        }
    };

    let ans = a
        .into_iter()
        .map(|x| x.to_string())
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
