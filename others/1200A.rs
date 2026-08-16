// Created: Aug 16 2026, 14:14:43
// Formatted with rustfmt.

fn main() {
    let n: usize = read();
    let mut rooms = vec![0i32; 10];
    for b in read::<String>().bytes() {
        if b == b'L' {
            for i in 0..10 {
                if rooms[i] == 0 {
                    rooms[i] = 1;
                    break;
                }
            }
        } else if b == b'R' {
            for i in (0..10).rev() {
                if rooms[i] == 0 {
                    rooms[i] = 1;
                    break;
                }
            }
        } else {
            rooms[(b - b'0') as usize] = 0;
        }
    }

    let ans = rooms
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("");
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
