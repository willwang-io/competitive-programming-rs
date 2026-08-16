// Created: Aug 16 2026, 01:21:52
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let a: Vec<i32> = (0..n).map(|_| read()).collect();
    let mut x = 0;
    let mut y = 0;

    for &i in &a {
        if i % 2 == 1 {
            x += 1;
        } else {
            y += 1;
        }
    }
    if x % 1 != y % 1 {
        println!("NO");
        return;
    } else {
        if x % 2 == 0 {
            println!("YES");
        } else {
            for i in 0..n {
                for j in 0..n {
                    if a[i] % 2 != a[j] % 2 && (a[i] - a[j]).abs() == 1 {
                        println!("YES");
                        return;
                    }
                }
            }
            println!("NO");
        }
    }
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
