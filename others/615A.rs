// Created: Aug 12 2026, 18:33:58
// Formatted with rustfmt.

fn main() {
    let n: usize = read();
    let m: usize = read();
    let mut cnt = vec![0; m];
    for _ in 0..n {
        for _ in 0..read::<usize>() {
            cnt[read::<usize>() - 1] = 1;
        }
    }
    let ok = cnt.iter().all(|&x| x != 0);
    if ok {
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
