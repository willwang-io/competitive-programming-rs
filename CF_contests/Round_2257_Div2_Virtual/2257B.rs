// Created: Aug 17 2026, 15:56:37
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let m: usize = read();
    let a: i64 = read();
    let _: Vec<i32> = (1..n).map(|_| read()).collect();
    let b: i64 = read();
    let _: Vec<i32> = (1..m).map(|_| read()).collect();

    if a + n as i64 >= b + m as i64 {
        println!("1");
    } else {
        println!("2");
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
