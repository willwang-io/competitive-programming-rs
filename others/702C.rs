// Created: Aug 20 2026, 19:17:02
// Formatted with rustfmt.

fn main() {
    let n: usize = read();
    let m: usize = read();
    let a: Vec<i64> = (0..n).map(|_| read()).collect();
    let b: Vec<i64> = (0..m).map(|_| read()).collect();
    let mut j = 0;
    let mut ans = 0;

    for x in a {
        while j + 1 < m && (x - b[j + 1]).abs() <= (x - b[j]).abs() {
            j += 1;
        }
        ans = ans.max((x - b[j]).abs());
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

