#![allow(unused)]

struct Solution;

impl Solution {
    /// [2839. Check if Strings Can be Made Equal With Operations I](https://leetcode.com/problems/check-if-strings-can-be-made-equal-with-operations-i)
    ///
    /// You are given two strings s1 and s2, both of length 4, consisting of lowercase English letters.
    ///
    /// You can apply the following operation on any of the two strings any number of times:
    /// - Choose any two indices i and j such that j - i = 2, then swap the two characters at those indices in the string.
    ///
    /// Return true if you can make the strings s1 and s2 equal, and false otherwise.
    ///
    pub fn can_be_equal(s1: String, s2: String) -> bool {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();

        // dbg!(s1.clone());
        // dbg!(s2.clone());

        // Check two swapable pairs
        for i in 0..2 {
            if !(s1[i] == s2[i] && s1[i + 2] == s2[i + 2]
                || s1[i] == s2[i + 2] && s1[i + 2] == s2[i])
            {
                return false;
            }
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ret = Solution::can_be_equal("abcd".to_owned(), "cdab".to_owned());
        assert_eq!(ret, true);
    }

    #[test]
    fn test_2() {
        let ret = Solution::can_be_equal("abcd".to_owned(), "dacb".to_owned());
        assert_eq!(ret, false);
    }
}
