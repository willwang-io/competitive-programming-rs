// Created: Aug 14 2026, 23:55:10
// Formatted with rustfmt.

fn main() {
    let n: usize = read();
    let m: usize = read();
    let mut ans = 0;
    for i in 0..n {
        for j in 0..m {
            let a: usize = read();
            let b: usize = read();
            if a + b > 0 {
                ans += 1;
            }
        }
    }
    println!("{ans}");
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
