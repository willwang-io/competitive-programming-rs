// Created: Aug 12 2026, 18:20:30
// Formatted with rustfmt.

fn main() {
    let n: usize = read();
    let k: usize = read();
    let ans = (0..n)
        .map(|i| {
            (0..n)
                .map(|j| {
                    if i == j {
                        k.to_string()
                    } else {
                        String::from("0")
                    }
                })
                .collect::<Vec<String>>()
                .join(" ")
        })
        .collect::<Vec<String>>()
        .join("\n");
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
