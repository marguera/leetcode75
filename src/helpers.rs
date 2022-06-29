pub fn vec_compare(va: Vec<i32>, vb: Vec<i32>) -> bool {
    let cond1 = va.len() == vb.len();
    let cond2 = va.iter().zip(vb.iter()).all(|(a, b)| a == b);
    cond1 && cond2
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_should_return_true() {
        let va = vec![2, 7, 11, 15];
        let vb = vec![2, 7, 11, 15];
        assert_eq!(vec_compare(va, vb), true);
    }

    #[test]
    fn it_should_return_false(){
        let va = vec![2, 8, 11, 15];
        let vb = vec![2, 7, 11, 15];
        assert_eq!(vec_compare(va, vb), false);
    }
}