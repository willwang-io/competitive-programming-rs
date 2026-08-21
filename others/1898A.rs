// Created: Aug 20 2026, 18:45:39
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let k: usize = read();
    let s = read::<String>().into_bytes();
    let b = s.iter().filter(|&&c| c == b'B').count();

    if b == k {
        println!("0");
        return;
    }

    let c = if b < k { b'B' } else { b'A' };
    let need = b.abs_diff(k);
    let mut cnt = 0;

    for (i, &x) in s.iter().enumerate().take(n) {
        if x != c {
            cnt += 1;
        }

        if cnt == need {
            println!("1\n{} {}", i + 1, c as char);
            return;
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
