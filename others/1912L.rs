// Created: Aug 19 2026, 17:26:49
// Formatted with rustfmt.

fn main() {
    let n: usize = read();
    let s = read::<String>().into_bytes();
    let total_l = s.iter().filter(|&&c| c == b'L').count();
    let total_o = n - total_l;
    let mut l = 0;
    let mut o = 0;

    for (i, &c) in s.iter().enumerate().take(n - 1) {
        if c == b'L' {
            l += 1;
        } else {
            o += 1;
        }

        if l != total_l - l && o != total_o - o {
            let ans = i + 1;
            println!("{ans}");
            return;
        }
    }

    println!("-1");
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

