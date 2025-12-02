pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        use std::collections::HashSet;

        let mut set = HashSet::<i32>::new();

        for num in nums {
            if set.contains(&num) {
                return true;
            }
            set.insert(num);
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn case_1() {
        let input = vec![1, 2, 3, 1];
        let expected = true;

        assert_eq!(expected, Solution::contains_duplicate(input));
    }

    #[test]
    fn case_2() {
        let input = vec![1, 2, 3, 4];
        let expected = false;

        assert_eq!(expected, Solution::contains_duplicate(input));
    }

    #[test]
    fn case_3() {
        let input = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
        let expected = true;

        assert_eq!(expected, Solution::contains_duplicate(input));
    }
}
