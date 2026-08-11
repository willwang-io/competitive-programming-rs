// Created: Aug 10 2026, 22:52:08
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let a: Vec<i32> = (0..n).map(|_| read()).collect();
    let mut ans: i32 = a.iter().sum();
    let mut ok = false;
    for i in 1..n {
        if a[i] == -1 && a[i - 1] == -1 {
            println!("{}", ans + 4);
            return;
        } else if a[i] * a[i - 1] < 0 {
            ok = true;
        }
    }
    if ok {
        println!("{ans}");
    } else {
        println!("{}", ans - 4);
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
