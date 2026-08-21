// Created: Jul 31 2026, 15:42:13
// Formatted with rustfmt.

fn solve() {
    let s: Vec<u8> = read::<String>().bytes().collect();
    let n = s.len();
    let mut l = s.iter().position(|&c| c == b'a').unwrap_or(n);
    let mut r = l;
    let mut ok = l < n;
    for i in 1..n {
        let c = b'a' + i as u8;
        if l > 0 && s[l - 1] == c {
            l -= 1;
        } else if r + 1 < n && s[r + 1] == c {
            r += 1;
        } else {
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
