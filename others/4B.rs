// Created: Aug 16 2026, 14:58:16
// Formatted with rustfmt.

fn main() {
    let d: usize = read();
    let sum_time: i32 = read();
    let mut a = vec![];
    let mut mn = 0;
    let mut mx = 0;

    for _ in 0..d {
        let l: i32 = read();
        let r: i32 = read();
        a.push((l, r));
        mn += l;
        mx += r;
    }

    if sum_time < mn || sum_time > mx {
        println!("NO");
        return;
    }

    let mut remaining = sum_time - mn;
    let mut ans = vec![];

    for (l, r) in a {
        let extra = remaining.min(r - l);
        ans.push(l + extra);
        remaining -= extra;
    }

    let ans = ans
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    println!("YES");
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
