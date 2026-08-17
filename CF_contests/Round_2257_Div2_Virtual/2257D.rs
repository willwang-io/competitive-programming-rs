// Created: Aug 17 2026, 16:10:27
// Formatted with rustfmt.

fn solve() {
    let s: i64 = read();
    let q: usize = read();
    let mut d = vec![];
    let mut i = 1;

    while i * i <= s {
        if s % i == 0 {
            d.push(i);
            if i * i != s {
                d.push(s / i);
            }
        }
        i += 1;
    }

    d.sort_unstable();

    let mut pref = vec![];
    let mut sum = 0;
    let mut prev = 0;

    for &v in &d {
        sum += (v - prev) * (s / v);
        pref.push(sum);
        prev = v;
    }

    for _ in 0..q {
        let x: i64 = read();
        let y: i64 = read();
        let r = d.partition_point(|&v| v < x);
        let j = d.partition_point(|&v| v < (s + y - 1) / y);

        let ans = if r < j {
            x * y
        } else {
            let left = if j == 0 { 0 } else { d[j - 1] * y };
            let mid = if r == j {
                0
            } else {
                pref[r - 1] - if j == 0 { 0 } else { pref[j - 1] }
            };
            let prev = if r == 0 { 0 } else { d[r - 1] };
            left + mid + (x - prev) * (s / d[r])
        };

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
