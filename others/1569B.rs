// Created: Aug 16 2026, 09:21:13
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let s: String = read();
    let two: Vec<usize> = s
        .bytes()
        .enumerate()
        .filter_map(|(i, c)| (c == b'2').then_some(i))
        .collect();

    if two.len() == 1 || two.len() == 2 {
        println!("NO");
        return;
    }

    let mut ans = vec![vec![b'='; n]; n];
    for (i, row) in ans.iter_mut().enumerate() {
        row[i] = b'X';
    }

    for i in 0..two.len() {
        let x = two[i];
        let y = two[(i + 1) % two.len()];
        ans[x][y] = b'+';
        ans[y][x] = b'-';
    }

    println!("YES");
    for row in ans {
        println!("{}", String::from_utf8(row).unwrap());
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
