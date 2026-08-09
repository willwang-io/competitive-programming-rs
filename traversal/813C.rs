// Created: Aug  2 2026, 11:42:13
// Formatted with rustfmt.

fn dfs(s: usize, adj: &[Vec<usize>]) -> Vec<i32> {
    let mut d = vec![-1; adj.len()];
    let mut st = vec![s];
    d[s] = 0;

    while let Some(v) = st.pop() {
        for &u in &adj[v] {
            if d[u] == -1 {
                d[u] = d[v] + 1;
                st.push(u);
            }
        }
    }

    d
}

fn main() {
    let n: usize = read();
    let x = read::<usize>() - 1;
    let mut adj = vec![vec![]; n];

    for _ in 1..n {
        let a = read::<usize>() - 1;
        let b = read::<usize>() - 1;
        adj[a].push(b);
        adj[b].push(a);
    }

    let a = dfs(0, &adj);
    let b = dfs(x, &adj);

    let mut ans = 0;
    for i in 0..n {
        if a[i] > b[i] {
            ans = ans.max(2 * a[i]);
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
