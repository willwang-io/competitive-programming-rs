// Created: Aug 14 2026, 02:11:29
// Formatted with rustfmt.

fn main() {
    let n: usize = read();
    let mut ans = vec![];
    for i in 0..30 {
        if n & (1 << i) != 0 {
            ans.push((i + 1).to_string());
        }
    }
    let ans = ans.into_iter().rev().collect::<Vec<_>>().join(" ");
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
