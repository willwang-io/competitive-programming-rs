// Created: Aug  5 2026, 11:24:28
// Formatted with rustfmt.

struct Dsu {
    parent: Vec<usize>,
    size: Vec<usize>,
    components: usize,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            components: n,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let p = self.parent[x];
            self.parent[x] = self.find(p);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let mut a = self.find(a);
        let mut b = self.find(b);

        if a == b {
            return false;
        }

        if self.size[a] < self.size[b] {
            std::mem::swap(&mut a, &mut b);
        }

        self.parent[b] = a;
        self.size[a] += self.size[b];
        self.components -= 1;
        true
    }

    fn same(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    fn size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        self.size[root]
    }

    fn components(&self) -> usize {
        self.components
    }
}

fn main() {
    let n: usize = read();
    let m: usize = read();

    let mut dsu = Dsu::new(n + 1);

    for i in 0..m {
        let k: usize = read();
        let a: Vec<usize> = (0..k).map(|_| read()).collect();
        for i in 1..k {
            dsu.union(a[0], a[i]);
        }
    }

    let ans = (1..=n)
        .map(|i| dsu.size(i).to_string())
        .collect::<Vec<_>>()
        .join(" ");
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
