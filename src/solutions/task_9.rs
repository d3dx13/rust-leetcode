#![allow(unused)]
struct Solution;

impl Solution {
    /// [9. Palindrome Number](https://leetcode.com/problems/palindrome-number)
    ///
    /// Given an integer x, return true if x is a palindrome, and false otherwise.
    ///
    pub fn is_palindrome(x: i32) -> bool {
        if x.to_string() == x.to_string().chars().rev().collect::<String>() {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::is_palindrome(121), true);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::is_palindrome(-121), false);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::is_palindrome(10), false);
    }
}
