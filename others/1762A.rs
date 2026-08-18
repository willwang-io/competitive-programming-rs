// Created: Aug 18 2026, 09:52:02
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let a: Vec<i32> = (0..n).map(|_| read()).collect();

    if a.iter().sum::<i32>() % 2 == 0 {
        println!("0");
        return;
    }

    let mut ans = i32::MAX;

    for mut x in a {
        let p = x % 2;
        let mut cnt = 0;
        while x % 2 == p {
            x /= 2;
            cnt += 1;
        }
        ans = ans.min(cnt);
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
