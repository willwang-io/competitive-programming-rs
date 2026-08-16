// Created: Aug 16 2026, 15:04:16
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let k: usize = read();
    let mut a: Vec<i64> = (0..n).map(|_| read()).collect();
    a.sort_unstable();

    let mut best = None;
    let mut current = None;
    let mut i = 0;

    while i < n {
        let mut j = i + 1;
        while j < n && a[j] == a[i] {
            j += 1;
        }

        if j - i >= k {
            current = match current {
                Some((l, r)) if a[i] == r + 1 => Some((l, a[i])),
                _ => Some((a[i], a[i])),
            };

            if let Some((l, r)) = current {
                if best.is_none_or(|(bl, br)| r - l > br - bl) {
                    best = Some((l, r));
                }
            }
        } else {
            current = None;
        }

        i = j;
    }

    if let Some((l, r)) = best {
        println!("{l} {r}");
    } else {
        println!("-1");
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
