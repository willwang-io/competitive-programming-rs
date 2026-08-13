// Created: Aug 12 2026, 19:53:14
// Formatted with rustfmt.

fn main() {
    let n: usize = read();
    let a: Vec<Vec<u8>> = (0..n).map(|_| read::<String>().into_bytes()).collect();

    let mut ans = 0;

    for row in &a {
        let cnt = row.iter().filter(|&&x| x == b'C').count();
        ans += cnt * (cnt - 1) / 2;
    }

    for j in 0..n {
        let mut cnt = 0;
        for i in 0..n {
            if a[i][j] == b'C' {
                cnt += 1;
            }
        }
        ans += cnt * (cnt - 1) / 2;
    }

    println!("{ans}");
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
