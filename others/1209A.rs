// Created: Aug 11 2026, 19:00:31
// Formatted with rustfmt.

fn main() {
    let n: usize = read();
    let a: Vec<usize> = (0..n).map(|_| read()).collect();

    let mut cnt = [0; 201];
    for x in a {
        cnt[x] = 1;
    }

    let mut ans = 0;
    for i in 1..=100 {
        if cnt[i] == 0 {
            continue;
        }
        ans += 1;
        for j in (i..=100).step_by(i) {
            cnt[j] = 0;
        }
    }
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
