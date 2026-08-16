// Created: Aug 16 2026, 09:45:13
// Formatted with rustfmt.

fn solve() {
    let m: usize = read();
    let mut last = vec![0usize; 50001];

    for i in 1..=m {
        let n: usize = read();
        for _ in 0..n {
            let x: usize = read();
            last[x] = i;
        }
    }

    let mut ans = vec![0usize; m];
    for x in 1..last.len() {
        if last[x] != 0 {
            ans[last[x] - 1] = x;
        }
    }

    if ans.contains(&0) {
        println!("-1");
    } else {
        let ans = ans
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        println!("{ans}");
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

