// Created: Aug 16 2026, 15:00:05
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let mut a = vec![];
    let mut cnt = vec![0; 101];

    for _ in 0..n {
        let x: usize = read();
        a.push(x);
        cnt[x] += 1;
    }

    let dup: Vec<usize> = (1..=100).filter(|&x| cnt[x] >= 2).take(2).collect();

    if dup.len() < 2 {
        println!("-1");
        return;
    }

    let mut used = [false; 2];
    let mut ans = vec![];

    for x in a {
        if x == dup[0] && !used[0] {
            ans.push(2);
            used[0] = true;
        } else if x == dup[1] && !used[1] {
            ans.push(3);
            used[1] = true;
        } else {
            ans.push(1);
        }
    }

    let ans = ans
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    println!("{ans}");
}

fn main() {
    let t: usize = read();
    for _ in 0..t {
        solve();
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
