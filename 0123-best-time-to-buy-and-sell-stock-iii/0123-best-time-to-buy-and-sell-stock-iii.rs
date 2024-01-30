use std::cmp;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        
        let n = prices.len();
        if n <= 1 {
            return 0;
        }
        
        let mut dp = vec![vec![0; 3]; n];
        
        for t in 1..=2 {
            let mut max_diff = -prices[0];
            for i in 1..n {
                            dp[i][t] = cmp::max(dp[i - 1][t], prices[i] + max_diff);
            max_diff = cmp::max(max_diff, dp[i][t - 1] - prices[i]);

            }
        }
        return dp[n-1][2];
    }
}