// Created: Aug 14 2026, 23:52:30
// Formatted with rustfmt.

fn main() {
    let n: usize = read();
    let a: Vec<i32> = (0..n).map(|_| read()).collect();

    let mut ans = vec![];

    for i in 0..n {
        let mut s = 1;
        for j in 0..n {
            if a[j] > a[i] {
                s += 1;
            }
        }
        ans.push(s.to_string());
    }

    let ans = ans.join(" ");
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
