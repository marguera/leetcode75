use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut set = HashSet::new();
    for n in nums.iter() {
        if set.contains(n){
            return true;
        }
        set.insert(n);
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_return_true(){
        let nums = vec![1, 2, 3, 1];
        assert_eq!(contains_duplicate(nums), true);
    }

    #[test]
    fn it_should_return_true_if_many_duplicates(){
        let nums = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
        assert_eq!(contains_duplicate(nums), true);
    }

    #[test]
    fn it_should_return_false() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(contains_duplicate(nums), false);
    }

    #[test]
    fn it_should_return_false_if_empty() {
        let nums = vec![];
        assert_eq!(contains_duplicate(nums), false);
    }

    #[test]
    fn it_should_return_false_if_one_element() {
        let nums = vec![1];
        assert_eq!(contains_duplicate(nums), false);
    }
}