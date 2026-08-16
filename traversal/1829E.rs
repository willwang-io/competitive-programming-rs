// Created: Aug 16 2026, 09:14:54
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let m: usize = read();
    let mut a = vec![vec![0i32; m]; n];

    for row in &mut a {
        for x in row {
            *x = read();
        }
    }

    let d = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut ans = 0;

    for i in 0..n {
        for j in 0..m {
            if a[i][j] == 0 {
                continue;
            }

            let mut stack = vec![(i, j)];
            let mut sum = a[i][j];
            a[i][j] = 0;

            while let Some((x, y)) = stack.pop() {
                for &(dx, dy) in &d {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;

                    if nx >= 0 && ny >= 0 && nx < n as i32 && ny < m as i32 {
                        let nx = nx as usize;
                        let ny = ny as usize;

                        if a[nx][ny] != 0 {
                            sum += a[nx][ny];
                            a[nx][ny] = 0;
                            stack.push((nx, ny));
                        }
                    }
                }
            }

            ans = ans.max(sum);
        }
    }

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
