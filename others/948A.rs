// Created: Aug 16 2026, 09:30:07
// Formatted with rustfmt.

fn main() {
    let r: usize = read();
    let c: usize = read();
    let mut a: Vec<Vec<u8>> = (0..r).map(|_| read::<String>().into_bytes()).collect();

    for i in 0..r {
        for j in 0..c {
            if a[i][j] == b'.' {
                a[i][j] = b'D';
            }

            if a[i][j] == b'W'
                && ((i > 0 && a[i - 1][j] == b'S')
                    || (i + 1 < r && a[i + 1][j] == b'S')
                    || (j > 0 && a[i][j - 1] == b'S')
                    || (j + 1 < c && a[i][j + 1] == b'S'))
            {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
    for row in a {
        println!("{}", String::from_utf8(row).unwrap());
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
