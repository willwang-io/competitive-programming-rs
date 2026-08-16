// Created: Aug 16 2026, 01:30:55
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let a: Vec<usize> = (0..n).map(|_| read()).collect();

    let mut cnt = vec![0; 100001];
    for &x in &a {
        cnt[x] += 1;
    }

    let mut x = 0;
    let mut y = 0;
    for i in cnt {
        if i == 0 {
            continue;
        }
        if x == 0 {
            x = i;
        } else if y == 0 {
            y = i;
        } else {
            println!("NO");
            return;
        }
    }
    let x = x as i32;
    let y = y as i32;
    if (x == 0 && y != 0) || (x != 0 && y == 0) || (x - y).abs() <= 1 {
        println!("YES");
    } else {
        println!("NO");
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
