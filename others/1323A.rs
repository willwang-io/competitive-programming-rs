// Created: Aug 11 2026, 14:05:23
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let a: Vec<i32> = (0..n).map(|_| read()).collect();
    let mut odd = vec![];
    for i in 0..n {
        if a[i] % 2 == 0 {
            println!("1\n{}", i + 1);
            return;
        } else {
            odd.push(i + 1);
        }
        if odd.len() == 2 {
            println!("2\n{} {}", odd[0], odd[1]);
            return;
        }
    }
    println!("-1");
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
