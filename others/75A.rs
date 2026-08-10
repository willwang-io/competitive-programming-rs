// Created: Aug  9 2026, 17:26:17
// Formatted with rustfmt.

fn main() {
    let a: i64 = read();
    let b: i64 = read();
    let c = a + b;

    let mut sa = a.to_string();
    let mut sb = b.to_string();
    let mut sc = c.to_string();

    sa.retain(|c| c != '0');
    sb.retain(|c| c != '0');
    sc.retain(|c| c != '0');

    let a = sa.parse::<i64>().unwrap();
    let b = sb.parse::<i64>().unwrap();
    let c = sc.parse::<i64>().unwrap();

    if a + b == c {
        println!("YES");
    } else {
        println!("NO");
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

