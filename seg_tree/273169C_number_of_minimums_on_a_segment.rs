// Created: Aug 12 2026, 11:38:51
// Formatted with rustfmt.

#[derive(Clone, Copy)]
struct Node {
    mn: i64,
    cnt: usize,
}

fn merge(a: Node, b: Node) -> Node {
    if a.mn < b.mn {
        a
    } else if a.mn > b.mn {
        b
    } else {
        Node {
            mn: a.mn,
            cnt: a.cnt + b.cnt,
        }
    }
}

struct SegTree<T, F> {
    n: usize,
    tree: Vec<T>,
    e: T,
    op: F,
}

impl<T: Copy, F: Fn(T, T) -> T> SegTree<T, F> {
    fn new(a: &[T], e: T, op: F) -> Self {
        let n = a.len().next_power_of_two();
        let mut tree = vec![e; n * 2];
        tree[n..n + a.len()].copy_from_slice(a);

        for i in (1..n).rev() {
            tree[i] = op(tree[i * 2], tree[i * 2 + 1]);
        }

        Self { n, tree, e, op }
    }

    fn set(&mut self, mut p: usize, x: T) {
        p += self.n;
        self.tree[p] = x;

        while p > 1 {
            p /= 2;
            self.tree[p] = (self.op)(self.tree[p * 2], self.tree[p * 2 + 1]);
        }
    }

    fn get(&self, p: usize) -> T {
        self.tree[self.n + p]
    }

    fn query(&self, mut l: usize, mut r: usize) -> T {
        l += self.n;
        r += self.n;

        let mut left = self.e;
        let mut right = self.e;

        while l < r {
            if l % 2 == 1 {
                left = (self.op)(left, self.tree[l]);
                l += 1;
            }
            if r % 2 == 1 {
                r -= 1;
                right = (self.op)(self.tree[r], right);
            }
            l /= 2;
            r /= 2;
        }

        (self.op)(left, right)
    }
}

fn main() {
    let n: usize = read();
    let m: usize = read();
    let a: Vec<Node> = (0..n).map(|_| Node { mn: read(), cnt: 1 }).collect();

    let mut seg_tree = SegTree::new(
        &a,
        Node {
            mn: i64::MAX,
            cnt: 0,
        },
        merge,
    );

    for _ in 0..m {
        let opr: usize = read();
        if opr == 1 {
            let i: usize = read();
            let v: i64 = read();
            seg_tree.set(i, Node { mn: v, cnt: 1 });
        } else {
            let l: usize = read();
            let r: usize = read();
            let ans = seg_tree.query(l, r);
            println!("{} {}", ans.mn, ans.cnt);
        }
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
