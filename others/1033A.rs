// Created: Aug 19 2026, 17:07:19
// Formatted with rustfmt.

fn main() {
    let _: i32 = read();
    let ax: i32 = read();
    let ay: i32 = read();
    let bx: i32 = read();
    let by: i32 = read();
    let cx: i32 = read();
    let cy: i32 = read();

    let ok = (bx < ax) == (cx < ax) && (by < ay) == (cy < ay);
    let ans = if ok { "YES" } else { "NO" };

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
