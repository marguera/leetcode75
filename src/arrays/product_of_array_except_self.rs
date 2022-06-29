// https://leetcode.com/problems/product-of-array-except-self/

pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {

    if nums.len() == 0 {
        return vec![];
    }

    if nums.len() == 1 {
        return vec![0];
    }
  
    // Allocate memory for temporary arrays left[] and right[]
    let mut left = vec![0; nums.len()];
    let mut right = vec![0; nums.len()];
  
    // Allocate memory for the product array
    let mut prod = vec![1; nums.len()];
  
    // Left most element of left array is always 1
    left[0] = 1;
  
    // Right most element of right array is always 1
    right[nums.len() - 1] = 1;
    
    // Construct the left array
    for i in 1..nums.len() {
        left[i] = nums[i - 1] * left[i - 1];
    }

    // Construct the right array
    for j in (0..nums.len()-1).rev() {
        right[j] = nums[j + 1] * right[j + 1];
    }

    // Construct the product array using
    // left[] and right[]
    for i in 0..nums.len() {
        prod[i] = left[i] * right[i];
    }
  
    return prod;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_return_the_product() {
        assert_eq!(product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
        assert_eq!(product_except_self(vec![-1, 1, 0, -3, 3]), vec![0, 0, 9, 0, 0]);
    }

    #[test]
    fn it_should_return_empty() {
        assert_eq!(product_except_self(vec![]), vec![]);
    }
}
