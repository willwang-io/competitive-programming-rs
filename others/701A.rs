// Created: Aug 16 2026, 13:59:46
// Formatted with rustfmt.

fn main() {
    let n: usize = read();
    let mut a: Vec<(_, _)> = (1..=n).map(|i| (read::<i32>(), i)).collect();
    a.sort_unstable();
    for i in 0..n / 2 {
        println!("{} {}", a[i].1, a[n - i - 1].1);
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
