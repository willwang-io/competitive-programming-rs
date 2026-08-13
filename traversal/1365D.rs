// Created: Aug 13 2026, 10:48:19
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let m: usize = read();
    let mut a: Vec<Vec<u8>> = (0..n).map(|_| read::<String>().into_bytes()).collect();

    let d = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    for i in 0..n {
        for j in 0..m {
            if a[i][j] != b'B' {
                continue;
            }

            for &(di, dj) in &d {
                let x = i as i32 + di;
                let y = j as i32 + dj;
                if x < 0 || y < 0 || x >= n as i32 || y >= m as i32 {
                    continue;
                }
                let x = x as usize;
                let y = y as usize;
                if a[x][y] == b'G' {
                    println!("NO");
                    return;
                }
                if a[x][y] == b'.' {
                    a[x][y] = b'#';
                }
            }
        }
    }

    let mut vis = vec![vec![false; m]; n];
    let mut st = vec![];

    if a[n - 1][m - 1] != b'#' {
        vis[n - 1][m - 1] = true;
        st.push((n - 1, m - 1));
    }

    while let Some((i, j)) = st.pop() {
        for &(di, dj) in &d {
            let x = i as i32 + di;
            let y = j as i32 + dj;
            if x < 0 || y < 0 || x >= n as i32 || y >= m as i32 {
                continue;
            }
            let x = x as usize;
            let y = y as usize;
            if a[x][y] != b'#' && !vis[x][y] {
                vis[x][y] = true;
                st.push((x, y));
            }
        }
    }

    for i in 0..n {
        for j in 0..m {
            if (a[i][j] == b'G' && !vis[i][j]) || (a[i][j] == b'B' && vis[i][j]) {
                println!("NO");
                return;
            }
        }
    }
    println!("YES");
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
