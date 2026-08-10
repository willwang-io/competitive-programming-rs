// Created: Aug  9 2026, 18:51:29
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let m: usize = read();
    let a: Vec<Vec<u8>> = (0..n).map(|_| read::<String>().into_bytes()).collect();
    let mut pos = vec![];
    for i in 0..n {
        for j in 0..m {
            if a[i][j] == b'R' {
                pos.push((i, j));
            }
        }
    }

    for &(x1, y1) in &pos {
        let mut ok = true;
        for &(x2, y2) in &pos {
            let dx = x2 as i32 - x1 as i32;
            let dy = y2 as i32 - y1 as i32;
            if dx < 0 || dy < 0 {
                ok = false;
                break;
            }
        }
        if ok {
            println!("YES");
            return;
        }
    }

    println!("NO");
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
