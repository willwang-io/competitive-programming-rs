// Created: Aug 16 2026, 14:23:58
// Formatted with rustfmt.

fn main() {
    let n: usize = read();
    let k: usize = read();
    let x: i32 = read();
    let a: Vec<i32> = (0..n).map(|_| read()).collect();
    let mut sum: i32 = a.iter().sum();
    let mut ans = sum;
    for i in 0..k {
        sum = sum - a[n - i - 1] + x;
        ans = ans.min(sum);
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
