// Created: Aug 11 2026, 01:05:01
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let a: Vec<Vec<u8>> = (0..n - 2).map(|_| read::<String>().into_bytes()).collect();
    let mut ans = vec![a[0][0]];
    let mut ok = true;
    for i in 1..a.len() {
        if a[i][0] != a[i - 1][1] {
            ok = false;
            ans.push(a[i - 1][1]);
        }
        ans.push(a[i][0]);
    }
    ans.push(a[n - 3][1]);
    if ok {
        ans.push(b'a');
    }
    let ans = String::from_utf8(ans).unwrap();
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
