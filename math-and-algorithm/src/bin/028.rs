use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [isize; n],
    }

    let mut dp = vec![0; n + 1];

    for i in 1..=n {
        let v = h[i - 1];
        if i == 1 {
            dp[i] = 0;
        }

        if i == 2 {
            dp[i] = (h[i - 2] - v).abs();
        }

        if i >= 3 {
            let dp1 = dp[i - 1] + (h[i - 2] - v).abs();
            let dp2 = dp[i - 2] + (h[i - 3] - v).abs();
            dp[i] = dp1.min(dp2);
        }
    }

    println!("{}", dp[n]);
}
