// Created: Aug 16 2026, 01:37:25
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let a: Vec<i64> = (0..n).map(|_| read()).collect();

    let mut ans = 0;
    for k in 1..=n {
        if n % k != 0 {
            continue;
        }
        let mut b = vec![];
        let mut cur = 0i64;
        for i in 0..n {
            cur += a[i];
            if (i + 1) % k == 0 {
                b.push(cur);
                cur = 0;
            }
        }
        b.sort_unstable();
        ans = ans.max(b[b.len() - 1] - b[0]);
    }

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
