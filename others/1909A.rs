// Created: Aug 20 2026, 18:39:47
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let mut dir = [false; 4];

    for _ in 0..n {
        let x: i32 = read();
        let y: i32 = read();

        dir[0] |= x > 0;
        dir[1] |= x < 0;
        dir[2] |= y > 0;
        dir[3] |= y < 0;
    }

    if dir.into_iter().all(|x| x) {
        println!("NO");
    } else {
        println!("YES");
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
