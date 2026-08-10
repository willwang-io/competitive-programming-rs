// Created: Aug  9 2026, 15:36:35
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let q: usize = read();
    let a: Vec<i64> = (0..n).map(|_| read()).collect();

    let mut even_cnt = a.iter().filter(|&&x| x % 2 == 0).count() as i64;
    let mut odd_cnt = n as i64 - even_cnt;
    let mut total: i64 = a.iter().sum();

    for _ in 0..q {
        let opr: usize = read();
        let x: i64 = read();
        if opr == 0 {
            total += even_cnt * x;
            if x % 2 == 1 {
                odd_cnt += even_cnt;
                even_cnt = 0;
            }
        } else {
            total += odd_cnt * x;
            if x % 2 == 1 {
                even_cnt += odd_cnt;
                odd_cnt = 0;
            }
        }
        println!("{total}");
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
