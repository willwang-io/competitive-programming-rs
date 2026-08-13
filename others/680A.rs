// Created: Aug 12 2026, 21:25:58
// Formatted with rustfmt.

fn main() {
    let a: Vec<usize> = (0..5).map(|_| read()).collect();
    let mut sum: usize = a.iter().sum();
    let mut ans = sum;
    let mut cnt = vec![0; 101];
    for x in a {
        cnt[x] += 1;
    }
    for i in 1..=100 {
        if cnt[i] >= 3 {
            ans = ans.min(sum - i * 3);
        } else if cnt[i] >= 2 {
            ans = ans.min(sum - i * 2);
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
