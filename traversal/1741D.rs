// Created: Aug 20 2026, 19:21:09
// Formatted with rustfmt.

fn dfs(a: &[i32]) -> Option<(i32, i32, i32)> {
    if a.len() == 1 {
        return Some((a[0], a[0], 0));
    }

    let (l, r) = a.split_at(a.len() / 2);
    let (lmin, lmax, x) = dfs(l)?;
    let (rmin, rmax, y) = dfs(r)?;

    if lmax + 1 == rmin {
        Some((lmin, rmax, x + y))
    } else if rmax + 1 == lmin {
        Some((rmin, lmax, x + y + 1))
    } else {
        None
    }
}

fn solve() {
    let m: usize = read();
    let a: Vec<i32> = (0..m).map(|_| read()).collect();
    let ans = dfs(&a).map_or(-1, |x| x.2);
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
