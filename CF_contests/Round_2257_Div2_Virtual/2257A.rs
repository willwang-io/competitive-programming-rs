// Created: Aug 17 2026, 09:50:15
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let m: usize = read();
    let mut has = [false; 26];

    for _ in 0..n {
        let s: String = read();
        has[(s.as_bytes()[0] - b'a') as usize] = true;
    }

    let mut ok = true;

    for _ in 0..m {
        let s: String = read();
        if !s.bytes().all(|c| has[(c - b'A') as usize]) {
            ok = false;
        }
    }

    if ok {
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
