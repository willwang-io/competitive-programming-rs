// Created: Aug 19 2026, 17:15:12
// Formatted with rustfmt.

fn main() {
    let n: usize = read();
    let mut a = 0i64;
    let mut b = 0i64;
    let mut c = 0i64;

    for _ in 0..n {
        a += read::<i64>();
    }
    for _ in 0..n - 1 {
        b += read::<i64>();
    }
    for _ in 0..n - 2 {
        c += read::<i64>();
    }

    let first = a - b;
    let second = b - c;
    println!("{first}");
    println!("{second}");
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
