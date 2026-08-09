// Created: Aug  9 2026, 09:00:02
// Formatted with rustfmt.

use std::io::{BufRead, Write};

fn first() {
    let n: i64 = read();
    let a: Vec<Vec<u8>> = (0..n).map(|_| read::<String>().into_bytes()).collect();

    let mut w = 0;
    let mut sr = 0;
    let mut sc = 0;

    for r in 0..n {
        for c in 0..n {
            if a[r as usize][c as usize] == b'#' {
                w += 1;
                sr = (sr + r) % n;
                sc = (sc + c) % n;
            }
        }
    }

    let rx = read::<i64>() - 1;
    let cx = read::<i64>() - 1;
    let dr = (w * rx - sr).rem_euclid(n);
    let dc = (w * cx - sc).rem_euclid(n);

    if dr == 0 && dc == 0 {
        println!("1 1 1 1");
        return;
    }

    for r in 0..n {
        for c in 0..n {
            let nr = (r + dr) % n;
            let nc = (c + dc) % n;
            if a[r as usize][c as usize] == b'#' && a[nr as usize][nc as usize] == b'.' {
                println!("{} {} {} {}", r + 1, c + 1, nr + 1, nc + 1);
                return;
            }
        }
    }
}

fn second() {
    let n: i64 = read();
    let a: Vec<Vec<u8>> = (0..n).map(|_| read::<String>().into_bytes()).collect();

    let mut w = 0;
    let mut sr = 0;
    let mut sc = 0;

    for r in 0..n {
        for c in 0..n {
            if a[r as usize][c as usize] == b'#' {
                w += 1;
                sr = (sr + r) % n;
                sc = (sc + c) % n;
            }
        }
    }

    let inv = |mut a: i64, mut b: i64| -> i64 {
        let n = b;
        let mut x = 1;
        let mut y = 0;

        while b != 0 {
            let q = a / b;
            (a, b) = (b, a - q * b);
            (x, y) = (y, x - q * y);
        }

        x.rem_euclid(n)
    };

    let x = inv(w, n);
    println!("{} {}", sr * x % n + 1, sc * x % n + 1);
}

fn main() {
    let s: String = read();
    let t: usize = read();

    for _ in 0..t {
        if s == "first" {
            first();
        } else {
            second();
        }
        std::io::stdout().flush().unwrap();
    }
}

struct Scanner {
    input: std::io::BufReader<std::io::Stdin>,
    tokens: Vec<String>,
}

impl Scanner {
    fn read<T: std::str::FromStr>(&mut self) -> T
    where
        T::Err: std::fmt::Debug,
    {
        loop {
            if let Some(s) = self.tokens.pop() {
                return s.parse().unwrap();
            }

            let mut line = String::new();
            self.input.read_line(&mut line).unwrap();
            self.tokens = line
                .split_ascii_whitespace()
                .rev()
                .map(String::from)
                .collect();
        }
    }
}

thread_local! {
    pub static INPUT: std::cell::RefCell<Scanner> = std::cell::RefCell::new(Scanner {
        input: std::io::BufReader::new(std::io::stdin()),
        tokens: Vec::new(),
    });
}

pub fn read<T: std::str::FromStr>() -> T
where
    T::Err: std::fmt::Debug,
{
    INPUT.with(|input| input.borrow_mut().read())
}
