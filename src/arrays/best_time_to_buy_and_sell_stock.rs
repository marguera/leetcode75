// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut min = i32::MAX;
    let mut profit = 0;
    for price in prices.into_iter() {
        if min > price {
            min = price
        } else {
            if price - min > profit {
                profit = price - min;
            };
        }
    }
    return profit;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_should_return_max_profit() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(max_profit(prices), 5);
    }
    
    #[test]
    fn it_should_return_0_if_no_profit() {
        let prices = vec![7, 6, 4, 3, 1];
        assert_eq!(max_profit(prices), 0);
    }

    #[test]
    fn it_should_return_0_if_no_prices() {
        let prices = vec![];
        assert_eq!(max_profit(prices), 0);
    }
}