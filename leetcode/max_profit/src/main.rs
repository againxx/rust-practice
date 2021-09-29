struct Solution;

impl Solution {
    fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {
            return 0;
        }
        let mut profile = 0;
        let mut buy_point = 0;
        for day in 0..prices.len() - 1 {
            if prices[day] > prices[day + 1] {
                if buy_point < day {
                    profile += prices[day] - prices[buy_point];
                }
                buy_point = day + 1;
            }
        }
        profile += prices.last().unwrap() - prices[buy_point];
        profile
    }
}

fn main() {
    let prices = vec![7,1,5,3,6,4];
    assert_eq!(Solution::max_profit(prices), 7);
}
