// Created: Aug 16 2026, 09:32:36
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let m: usize = read();
    let mut adj = vec![Vec::new(); n];

    for _ in 0..m {
        let u = read::<usize>() - 1;
        let v = read::<usize>() - 1;
        adj[u].push(v);
        adj[v].push(u);
    }

    let mut color = vec![-1i8; n];
    let mut cnt = [0usize; 2];

    for i in 0..n {
        if color[i] != -1 {
            continue;
        }

        color[i] = 0;
        let mut stack = vec![i];

        while let Some(u) = stack.pop() {
            cnt[color[u] as usize] += 1;

            for &v in &adj[u] {
                if color[v] == -1 {
                    color[v] = color[u] ^ 1;
                    stack.push(v);
                }
            }
        }
    }

    let x = cnt[0].min(cnt[1]);
    let y = cnt[0].max(cnt[1]);
    let ans = (y - 1) / x;
    println!("{x} {ans}");
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
