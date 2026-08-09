// Created: Aug  2 2026, 11:26:00
// Formatted with rustfmt.

fn dfs(u: usize, par: Option<usize>, adj: &[Vec<(usize, i32)>]) -> i32 {
    let mut ans = 0;
    for &(v, c) in &adj[u] {
        if par != Some(v) {
            ans = ans.max(c + dfs(v, Some(u), adj));
        }
    }
    ans
}

fn main() {
    let n: usize = read();
    let mut adj = vec![vec![]; n];
    for _ in 1..n {
        let u = read::<usize>();
        let v = read::<usize>();
        let c = read::<i32>();
        adj[u].push((v, c));
        adj[v].push((u, c));
    }

    let ans = dfs(0, None, &adj);
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
