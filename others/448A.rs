// Created: Aug 10 2026, 22:38:00
// Formatted with rustfmt.

fn main() {
    let a = read::<i32>() + read::<i32>() + read::<i32>();
    let b = read::<i32>() + read::<i32>() + read::<i32>();
    let n: i32 = read();
    if (a + 4) / 5 + (b + 9) / 10 <= n {
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
