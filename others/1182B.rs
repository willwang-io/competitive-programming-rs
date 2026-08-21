// Created: Aug 20 2026, 19:14:09
// Formatted with rustfmt.

fn main() {
    let h: usize = read();
    let w: usize = read();
    let a: Vec<Vec<u8>> = (0..h).map(|_| read::<String>().into_bytes()).collect();

    if h < 3 || w < 3 {
        println!("NO");
        return;
    }

    let mut c = None;
    for i in 1..h - 1 {
        for j in 1..w - 1 {
            if a[i][j] == b'*'
                && a[i - 1][j] == b'*'
                && a[i + 1][j] == b'*'
                && a[i][j - 1] == b'*'
                && a[i][j + 1] == b'*'
            {
                c = Some((i, j));
            }
        }
    }

    let Some((x, y)) = c else {
        println!("NO");
        return;
    };

    let mut used = 1;
    for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let mut i = x as i32 + dx;
        let mut j = y as i32 + dy;

        while i >= 0 && i < h as i32 && j >= 0 && j < w as i32 && a[i as usize][j as usize] == b'*'
        {
            used += 1;
            i += dx;
            j += dy;
        }
    }

    let ans = if used == a.iter().flatten().filter(|&&x| x == b'*').count() {
        "YES"
    } else {
        "NO"
    };
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
