// Created: Aug  9 2026, 20:15:14
// Formatted with rustfmt.

fn main() {
    let mut a: Vec<i32> = (0..4).map(|_| read()).collect();
    a.sort_unstable();
    if a[0] + a[1] + a[2] == a[3] || a[0] + a[3] == a[1] + a[2] {
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
