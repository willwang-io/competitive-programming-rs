// Created: Aug 16 2026, 01:14:53
// Formatted with rustfmt.

fn main() {
    let n: usize = read();
    let m: usize = read();
    let mut a = vec![];
    for i in 0..n {
        a.push((read::<usize>(), i));
    }

    let mut adj = vec![vec![]; n];
    for _ in 0..m {
        let x = read::<usize>() - 1;
        let y = read::<usize>() - 1;
        adj[x].push(y);
        adj[y].push(x);
    }

    a.sort_unstable();

    let mut seen = vec![false; n];
    let mut ans = 0;

    for (i, x) in a {
        if !seen[x] {
            ans += i;
        }

        let mut stack = vec![x];
        while let Some(u) = stack.pop() {
            if seen[u] {
                continue;
            }
            seen[u] = true;
            for &v in &adj[u] {
                stack.push(v);
            }
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
