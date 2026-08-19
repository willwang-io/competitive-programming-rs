// Created: Aug 19 2026, 12:10:36
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let mut a: Vec<i32> = (0..n).map(|_| read()).collect();
    let mut b: Vec<i32> = (0..n).map(|_| read()).collect();

    for i in 0..n {
        if a[i] > b[i] {
            (a[i], b[i]) = (b[i], a[i]);
        }
    }

    if a[n - 1] == *a.iter().max().unwrap() && b[n - 1] == *b.iter().max().unwrap() {
        println!("YES");
    } else {
        println!("NO");
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
