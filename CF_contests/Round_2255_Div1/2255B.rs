// Created: Aug  9 2026, 08:41:51
// Formatted with rustfmt.

const MOD: i64 = 998_244_353;

fn pow(mut a: i64, mut b: i64) -> i64 {
    let mut ans = 1;
    while b > 0 {
        if b % 2 == 1 {
            ans = ans * a % MOD;
        }
        a = a * a % MOD;
        b >>= 1;
    }
    ans
}

fn comb(n: i64, k: i64) -> i64 {
    let k = k.min(n - k);
    let mut a = 1;
    let mut b = 1;
    for i in 1..=k {
        a = a * (n - k + i) % MOD;
        b = b * i % MOD;
    }
    a * pow(b, MOD - 2) % MOD
}

fn solve() {
    let n: usize = read();
    let s = read::<String>().into_bytes();
    let mut cnt = [0, 0];
    let mut run = [0, 0];
    let mut p = 2;

    for b in s {
        let x = (b - b'0') as usize;
        cnt[x] += 1;
        if x != p {
            run[x] += 1;
        }
        p = x;
    }

    let f = |c: i64, r: i64| -> i64 {
        if r == 0 {
            1
        } else {
            comb(c - 1, r - 1)
        }
    };

    let ans = f(cnt[0], run[0]) * f(cnt[1], run[1]) % MOD;
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
