// Created: Aug 19 2026, 17:09:34
// Formatted with rustfmt.

fn main() {
    let n: usize = read();
    let m: usize = read();
    let mut g = vec![vec![]; n];
    let mut deg = vec![0; n];

    for _ in 0..m {
        let a = read::<usize>() - 1;
        let b = read::<usize>() - 1;
        g[a].push(b);
        g[b].push(a);
        deg[a] += 1;
        deg[b] += 1;
    }

    let mut cur: Vec<usize> = (0..n).filter(|&i| deg[i] == 1).collect();
    let mut removed = vec![false; n];
    let mut ans = 0;

    while !cur.is_empty() {
        ans += 1;

        for &v in &cur {
            removed[v] = true;
        }

        let mut next = vec![];

        for v in cur {
            for &u in &g[v] {
                if !removed[u] {
                    deg[u] -= 1;
                    next.push(u);
                }
            }
        }

        next.sort_unstable();
        next.dedup();
        next.retain(|&v| deg[v] == 1);
        cur = next;
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
