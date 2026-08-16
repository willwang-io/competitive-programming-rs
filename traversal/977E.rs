// Created: Aug 16 2026, 09:05:26
// Formatted with rustfmt.

fn main() {
    let n: usize = read();
    let m: usize = read();
    let mut adj = vec![vec![]; n];

    for _ in 0..m {
        let u = read::<usize>() - 1;
        let v = read::<usize>() - 1;
        adj[u].push(v);
        adj[v].push(u);
    }

    let mut seen = vec![false; n];
    let mut ans = 0;

    for i in 0..n {
        if seen[i] {
            continue;
        }

        let mut stack = vec![i];
        let mut ok = true;
        seen[i] = true;

        while let Some(u) = stack.pop() {
            if adj[u].len() != 2 {
                ok = false;
            }
            for &v in &adj[u] {
                if !seen[v] {
                    seen[v] = true;
                    stack.push(v);
                }
            }
        }

        if ok {
            ans += 1;
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
