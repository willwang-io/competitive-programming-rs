// Created: Aug 15 2026, 22:52:27
// Formatted with rustfmt.

fn main() {
    let n: usize = read();
    let mut cnt = 0;
    for x in 0..50 {
        for y in 0..5 {
            cnt += 1 << x;
            if cnt >= n {
                let ans = if y == 0 {
                    "Sheldon"
                } else if y == 1 {
                    "Leonard"
                } else if y == 2 {
                    "Penny"
                } else if y == 3 {
                    "Rajesh"
                } else {
                    "Howard"
                };
                println!("{ans}");
                return;
            }
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
