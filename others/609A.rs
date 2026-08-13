// Created: Aug 12 2026, 18:38:27
// Formatted with rustfmt.

fn main() {
    let n: usize = read();
    let mut m: i32 = read();
    let mut a: Vec<i32> = (0..n).map(|_| read()).collect();

    a.sort_by(|x, y| y.cmp(x));
    for i in 0..n {
        m -= a[i];
        if m <= 0 {
            println!("{}", i + 1);
            return;
        }
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
