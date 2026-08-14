// Created: Aug 14 2026, 02:06:02
// Formatted with rustfmt.

fn main() {
    let n: usize = read();
    let d: usize = read();
    let a: Vec<Vec<u8>> = (0..d).map(|_| read::<String>().into_bytes()).collect();

    let mut ans = 0;
    let mut cnt = 0;
    for i in 0..d {
        let ok = a[i].iter().all(|&b| b == b'1');
        if ok {
            ans = ans.max(cnt);
            cnt = 0;
        } else {
            cnt += 1;
        }
    }

    println!("{}", cnt.max(ans));
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
