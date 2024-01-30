impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut totalAccumulatedProfit = 0;

        for price in 1..prices.len() {
            if prices[price] > prices[price - 1] {
                totalAccumulatedProfit += (prices[price] - prices[price - 1]);
            }
        }
        return totalAccumulatedProfit;
    }
}