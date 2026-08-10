// Created: Aug  9 2026, 19:08:02
// Formatted with rustfmt.

fn solve() {
    let s: String = read();

    let mut ok = s.ends_with("B");
    let mut cur = 0;
    for c in s.chars() {
        if c == 'A' {
            cur += 1;
        } else {
            cur -= 1;
        }
        if cur < 0 {
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
