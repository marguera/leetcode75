// https://leetcode.com/problems/maximum-subarray/
use std::cmp::Ordering;

pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    match nums.len().cmp(&1) {
        Ordering::Less => 0,
        Ordering::Equal => nums[0],
        Ordering::Greater => calculate(nums[0], nums[0], &nums[1..]),
    }
}

pub fn calculate(sum: i32, max: i32, nums: &[i32]) -> i32 {
    let partial_sum = sum.max(0) + nums[0];
    let max = max.max(partial_sum);
    if nums.len() > 1 { 
        return calculate(partial_sum, max, &nums[1..]);    
    } else {
        return max;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, -5]), 6);
    }
    
    #[test]
    fn test2() {
        assert_eq!(max_sub_array(vec![1]), 1);
    }
    
    #[test]
    fn test3() {
        assert_eq!(max_sub_array(vec![-1]), -1);
    }
    
    #[test]
    fn test4() {
        assert_eq!(max_sub_array(vec![5,4,-1,7,8]), 23);
    }
}