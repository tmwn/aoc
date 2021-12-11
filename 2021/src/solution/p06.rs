pub fn small(input: Vec<String>) -> i64 {
    solve(input, 80)
}

pub fn large(input: Vec<String>) -> i64 {
    solve(input, 256)
}

pub fn solve(input: Vec<String>, days: usize) -> i64 {
    let a = input[0]
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>();

    let mut dp = vec![0; 9];
    for i in a {
        dp[i] += 1;
    }
    for _ in 0..days {
        let mut ndp = vec![0; 9];
        for i in 1..9 {
            ndp[i - 1] += dp[i];
        }
        ndp[6] += dp[0];
        ndp[8] += dp[0];
        dp = ndp;
    }
    let mut res = 0;
    for i in dp {
        res += i;
    }
    res
}
