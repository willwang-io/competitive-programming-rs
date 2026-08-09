// Created: Aug  9 2026, 09:41:46
// Formatted with rustfmt.

use std::collections::BinaryHeap;

fn solve() {
    let n: usize = read();
    let a: Vec<i64> = (0..n).map(|_| read()).collect();

    let ok = |t: usize| -> bool {
        let mut h = BinaryHeap::from(a.to_vec());

        for k in (0..t).rev() {
            let Some(x) = h.pop() else {
                return true;
            };

            if k < i32::BITS as usize {
                let v = 1i64 << k;
                if x > v {
                    h.push(x - v);
                }
            }
        }

        h.is_empty()
    };

    let mut l = 0;
    let mut r = 1e10 as usize;

    while l + 1 < r {
        let m = l.midpoint(r);

        if ok(m) {
            r = m;
        } else {
            l = m;
        }
    }

    println!("{r}");
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
