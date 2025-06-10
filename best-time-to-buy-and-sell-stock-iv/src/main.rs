struct Solution;

impl Solution {
    fn recur(i: usize, k: i32, have_item: usize, prices: &Vec<i32>, dp: &mut Vec<Vec<Vec<i32>>>) -> i32 {

        if i == prices.len() { return if have_item == 1 && k > 0 { prices[i-1] } else { 0 }; }
        else if k == 0 { return 0; }

        let ki = k as usize - 1;
        if dp[i][ki][have_item] != -1 { return dp[i][ki][have_item]; }

        // buy/sell or do nothing

        let nothing = Self::recur(i+1, k, have_item, prices, dp);
        let buy_or_sell;

        // if have item to sell => sell
        if have_item == 1 { buy_or_sell = Self::recur(i+1, k-1, 0, prices, dp) + prices[i]; }

        // if not => buy
        else { buy_or_sell = Self::recur(i+1, k, 1, prices, dp) - prices[i]; }

        dp[i][ki][have_item] = std::cmp::max(buy_or_sell, nothing);
        dp[i][ki][have_item]
    }

    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        Self::recur(0,k,0,&prices, &mut vec![vec![vec![-1; 2]; k as usize]; prices.len()])
    }
}

fn main() {
    assert_eq!(Solution::max_profit(2, vec![2,4,1]), 2);
    assert_eq!(Solution::max_profit(2, vec![3,2,6,5,0,3]), 7);
}
