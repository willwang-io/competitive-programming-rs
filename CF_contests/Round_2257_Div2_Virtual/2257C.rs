// Created: Aug 17 2026, 16:03:19
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let _: Vec<i32> = (1..n).map(|_| read()).collect();

    let m: usize = read();
    let a: Vec<i32> = (0..m).map(|_| read()).collect();

    let skip = if a.contains(&1) {
        1
    } else {
        *a.iter().min().unwrap()
    };

    let mut ans = (m - 1).to_string();
    for x in a {
        if x != skip {
            ans.push(' ');
            ans.push_str(&x.to_string());
        }
    }

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
