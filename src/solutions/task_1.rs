#![allow(unused)]
struct Solution;

use std::collections::HashMap;

impl Solution {
    /// [1. Two Sum](https://leetcode.com/problems/two-sum/)
    ///
    /// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
    ///
    /// You may assume that each input would have exactly one solution, and you may not use the same element twice.
    ///
    /// You can return the answer in any order.
    ///
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut container: HashMap<i32, i32> = HashMap::new();

        for (index, &value) in nums.iter().enumerate() {
            container.insert(value, index as i32);
        }

        // println!("container={container:?}");

        for (index, &value) in nums.iter().enumerate() {
            let key = &(target - value);
            if container.contains_key(key) {
                let index_second = *container.get(key).unwrap();
                if index as i32 != index_second {
                    return vec![index as i32, index_second];
                }
            }
        }

        panic!("There are no such pairs");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ret = Solution::two_sum(vec![2, 7, 11, 15], 9);
        assert_eq!(ret, vec![0, 1]);
    }

    #[test]
    fn test_2() {
        let ret = Solution::two_sum(vec![3, 2, 4], 6);
        assert_eq!(ret, vec![1, 2]);
    }

    #[test]
    fn test_3() {
        let ret = Solution::two_sum(vec![3, 3], 6);
        assert_eq!(ret, vec![0, 1]);
    }
}
