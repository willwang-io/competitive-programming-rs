// Created: Aug 22 2026, 12:16:26
// Formatted with rustfmt.

fn main() {
    let n: usize = read();
    let mut a: Vec<i64> = (0..n).map(|_| read()).collect();
    let mut l = 0;

    while l + 1 < n && a[l] < a[l + 1] {
        l += 1;
    }

    if l + 1 == n {
        println!("yes");
        println!("1 1");
        return;
    }

    let mut r = l + 1;
    while r + 1 < n && a[r] > a[r + 1] {
        r += 1;
    }

    a[l..=r].reverse();

    if a.windows(2).all(|w| w[0] < w[1]) {
        println!("yes");
        println!("{} {}", l + 1, r + 1);
    } else {
        println!("no");
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
