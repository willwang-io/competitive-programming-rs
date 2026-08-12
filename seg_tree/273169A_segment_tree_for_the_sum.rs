// Created: Aug 12 2026, 11:13:17
// Formatted with rustfmt.

struct SegTree {
    n: usize,
    tree: Vec<i64>,
}

impl SegTree {
    fn new(a: &[i64]) -> Self {
        let n = a.len().next_power_of_two();
        let mut tree = vec![0; n * 2];
        tree[n..n + a.len()].copy_from_slice(a);
        for i in (1..n).rev() {
            tree[i] = tree[i * 2] + tree[i * 2 + 1];
        }

        Self { n, tree }
    }

    fn set(&mut self, mut p: usize, x: i64) {
        p += self.n;
        self.tree[p] = x;

        while p > 1 {
            p /= 2;
            self.tree[p] = self.tree[p * 2] + self.tree[p * 2 + 1];
        }
    }

    fn add(&mut self, p: usize, x: i64) {
        self.set(p, self.tree[self.n + p] + x);
    }

    fn sum(&self, mut l: usize, mut r: usize) -> i64 {
        l += self.n;
        r += self.n;
        let mut ans = 0;

        while l < r {
            if l % 2 == 1 {
                ans += self.tree[l];
                l += 1;
            }
            if r % 2 == 1 {
                r -= 1;
                ans += self.tree[r];
            }
            l /= 2;
            r /= 2;
        }

        ans
    }
}

fn main() {
    let n: usize = read();
    let m: usize = read();
    let a: Vec<i64> = (0..n).map(|_| read()).collect();

    let mut seg_tree = SegTree::new(&a);
    let mut ans = vec![];

    for _ in 0..m {
        let opr: usize = read();
        if opr == 1 {
            let i: usize = read();
            let v: i64 = read();
            seg_tree.set(i, v);
        } else {
            let l: usize = read();
            let r: usize = read();
            ans.push(seg_tree.sum(l, r));
        }
    }

    let ans = ans
        .iter()
        .map(|&x| x.to_string())
        .collect::<Vec<_>>()
        .join("\n");
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
