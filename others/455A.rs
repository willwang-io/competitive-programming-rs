const N: usize = 100001;

fn main() {
    let n: usize = read();
    let mut cnt = vec![0i64; N];
    for _ in 0..n {
        let x: i64 = read();
        cnt[x as usize] += x;
    }
    let mut dp = vec![-1; N];
    dp[0] = cnt[0];
    dp[1] = cnt[1];
    for i in 2..N {
        dp[i] = dp[i - 1].max(dp[i - 2] + cnt[i]);
    }
    println!("{}", dp[N - 1]);
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