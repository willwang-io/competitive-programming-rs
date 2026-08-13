// Created: Aug 12 2026, 10:25:39
// Formatted with rustfmt.

fn main() {
    let s: i32 = read();
    let v1: i32 = read();
    let v2: i32 = read();
    let t1: i32 = read();
    let t2: i32 = read();
    let x = 2 * t1 + s * v1;
    let y = 2 * t2 + s * v2;
    if x < y {
        println!("First");
    } else if x > y {
        println!("Second");
    } else {
        println!("Friendship");
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
