// Created: Aug 13 2026, 15:11:54
// Formatted with rustfmt.

use std::collections::HashMap;

fn main() {
    let n: usize = read();
    let a: Vec<i32> = (0..n).map(|_| read()).collect();

    let mut cnt: HashMap<i32, i32> = HashMap::new();
    for x in a {
        if x == 0 {
            continue;
        }
        *cnt.entry(x).or_default() += 1;
    }

    let mut ans = 0;
    for (k, v) in cnt {
        if v > 2 {
            println!("-1");
            return;
        }
        ans += v / 2;
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
