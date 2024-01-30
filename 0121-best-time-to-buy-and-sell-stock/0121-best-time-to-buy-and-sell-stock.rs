impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {

        // edge case: price array is empty
        if prices.is_empty() { return 0; }
        let mut min_price = prices[0];

        let mut maxProfit = 0;

        for price in prices.iter() {
            min_price = min_price.min(*price);
            maxProfit = maxProfit.max(*price - min_price);
        }

        return maxProfit;
        
    }
}