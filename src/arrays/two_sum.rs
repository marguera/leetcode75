// https://leetcode.com/problems/two-sum/

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut left:usize = 0;
    let mut right:usize = nums.len() - 1;
    let clone = nums.clone();
    let mut sorted = clone.iter().enumerate().collect::<Vec<_>>();
    sorted.sort_by_key(|k| k.1);
    
    let res: Vec<usize> = loop {
        let sum = sorted[left].1 + sorted[right].1;
        if sum == target {
            let l = sorted[left].0;
            let r = sorted[right].0;
            if l < r {
                break vec![l, r];
            } else {
                break vec![r, l];
            }
        }
        if sum < target {
            left += 1;  
        }
        if sum > target {
            right -= 1;  
        }
    };

    return vec![res[0] as i32, res[1] as i32];
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers;

    #[test]
    fn it_should_find_the_two_numbers() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(helpers::vec_compare(two_sum(nums, target), vec![0, 1]), true);
    }

    #[test]
    fn it_should_find_with_unsorted_lists() {
        let nums = vec![3, 2, 4];
        let target = 6;
        assert_eq!(helpers::vec_compare(two_sum(nums, target), vec![1, 2]), true);
    }

    #[test]
    fn it_should_find_with_same_numbers() {
        let nums = vec![3, 3];
        let target = 6;
        assert_eq!(helpers::vec_compare(two_sum(nums, target), vec![0, 1]), true);
    }

    #[test]
    fn it_should_find_with_negative_numbers() {
        let nums = vec![-1, -2, -3, -4, -5];
        let target = -8;
        assert_eq!(helpers::vec_compare(two_sum(nums, target), vec![2, 4]), true);
    }
}