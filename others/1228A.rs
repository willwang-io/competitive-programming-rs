// Created: Aug 16 2026, 14:03:32
// Formatted with rustfmt.

fn main() {
    let l: usize = read();
    let r: usize = read();

    let ok = |mut n: usize| -> bool {
        let mut cnt = vec![0; 10];
        while n > 0 {
            cnt[n % 10] += 1;
            n /= 10;
        }
        cnt.iter().all(|&x| x <= 1)
    };

    for i in l..=r {
        if ok(i) {
            println!("{i}");
            return;
        }
    }
    println!("-1");
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
