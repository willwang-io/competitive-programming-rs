// Created: Aug 10 2026, 00:33:07
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let a: Vec<usize> = (0..n).map(|_| read()).collect();

    let mut incr = 0;
    for i in 1..=n {
        if i + incr == a[i - 1] {
            incr += 1;
        }
    }
    println!("{}", n + incr);
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
