// Created: Aug 10 2026, 00:26:51
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let m: usize = read();
    let a: Vec<usize> = (0..n).map(|_| read()).collect();
    let b: Vec<usize> = (0..m).map(|_| read()).collect();
    let mut cnt = [0; 1001];
    for x in a {
        cnt[x] = 1;
    }

    for x in b {
        if cnt[x] != 0 {
            println!("YES\n1 {}", x);
            return;
        }
    }
    println!("NO");
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
