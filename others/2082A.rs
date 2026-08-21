// Created: Aug  3 2026, 01:15:01
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let m: usize = read();
    let mut rows = 0;
    let mut col = vec![0; m];
    for _ in 0..n {
        let s: String = read();
        let mut row = 0;
        for (j, c) in s.bytes().enumerate() {
            let x = (c - b'0') as i32;
            row ^= x;
            col[j] ^= x;
        }
        rows += row;
    }
    let ans = rows.max(col.iter().sum());
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
