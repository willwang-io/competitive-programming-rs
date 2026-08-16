// Created: Aug 14 2026, 23:57:39
// Formatted with rustfmt.

fn main() {
    let n: usize = read();
    let s: String = read();
    let mut ans = vec![];
    let mut cnt = 0;
    for b in s.bytes() {
        if b == b'B' {
            cnt += 1;
        } else {
            if cnt != 0 {
                ans.push(cnt);
            }
            cnt = 0;
        }
    }

    if cnt != 0 {
        ans.push(cnt);
    }
    let s = ans
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    println!("{}\n{}", ans.len(), s);
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
