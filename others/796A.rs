// Created: Aug 11 2026, 21:24:29
// Formatted with rustfmt.

fn main() {
    let n: usize = read();
    let m: i32 = read::<i32>() - 1;
    let k: i32 = read();
    let a: Vec<i32> = (0..n).map(|_| read()).collect();
    let mut ans = 500;
    for i in 0..n {
        if a[i] == 0 || a[i] > k {
            continue;
        }
        ans = ans.min((i as i32 - m).abs());
    }
    println!("{}", ans * 10);
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
