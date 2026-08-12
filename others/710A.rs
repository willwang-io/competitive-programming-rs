// Created: Aug 11 2026, 21:35:25
// Formatted with rustfmt.

fn main() {
    let s = read::<String>().into_bytes();
    let x = (s[0] - b'a') as i32;
    let y = (s[1] - b'0' - 1) as i32;
    let d = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (-1, 1),
        (1, -1),
        (-1, -1),
    ];
    let mut ans = 0;
    for (dx, dy) in d {
        let nx = x + dx;
        let ny = y + dy;
        if nx >= 0 && ny >= 0 && nx < 8 && ny < 8 {
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
