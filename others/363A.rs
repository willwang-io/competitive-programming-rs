// Created: Aug 16 2026, 13:38:45
// Formatted with rustfmt.

fn main() {
    let mut n: usize = read();
    if n == 0 {
        println!("O-|-OOOO");
        return;
    }
    while n > 0 {
        let mut r = n % 10;
        if r > 4 {
            print!("-O|");
            r -= 5;
        } else {
            print!("O-|");
        }
        println!("{}-{}", "O".repeat(r), "O".repeat(4 - r));
        n /= 10;
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
