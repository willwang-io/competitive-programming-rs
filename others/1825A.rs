// Created: Aug  9 2026, 15:29:33
// Formatted with rustfmt.

fn solve() {
    let s = read::<String>().into_bytes();

    let is_pal = |f: &[u8]| -> bool {
        for i in 0..s.len() / 2 {
            if s[i] != s[s.len() - i - 1] {
                return false;
            }
        }
        true
    };

    if !is_pal(&s) {
        println!("{}", s.len());
    } else {
        let ok = s.windows(2).any(|w| w[0] != w[1]);
        if ok {
            println!("{}", s.len() - 1);
        } else {
            println!("-1");
        }
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
