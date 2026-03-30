#![allow(unused)]

struct Solution;

impl Solution {
    /// [2840. Check if Strings Can be Made Equal With Operations II](https://leetcode.com/problems/check-if-strings-can-be-made-equal-with-operations-ii)
    ///
    /// You are given two strings s1 and s2, both of length n, consisting of lowercase English letters.
    ///
    /// You can apply the following operation on any of the two strings any number of times:
    /// - Choose any two indices i and j such that i < j and the difference j - i is even,
    /// then swap the two characters at those indices in the string.
    ///
    /// Return true if you can make the strings s1 and s2 equal, and false otherwise.
    ///
    pub fn check_strings(s1: String, s2: String) -> bool {
        let (v1_odd, v1_even) = Solution::count_lowercase_letters_odd_even(s1);
        let (v2_odd, v2_even) = Solution::count_lowercase_letters_odd_even(s2);

        // dbg!(v1_odd.clone(), v1_even.clone());
        // dbg!(v2_odd.clone(), v2_even.clone());

        v1_odd == v2_odd && v1_even == v2_even
    }

    pub fn count_lowercase_letters_odd_even(s: String) -> (Vec<i32>, Vec<i32>) {
        let mut res_odd = vec![0; ('z' as u8 - 'a' as u8 + 1) as usize];
        let mut res_even = vec![0; ('z' as u8 - 'a' as u8 + 1) as usize];

        for (i, c) in s.chars().enumerate() {
            if c < 'a' || c > 'z' {
                panic!("illegal number");
            }
            if i % 2 == 0 {
                res_odd[(c as u8 - 'a' as u8) as usize] += 1;
            } else {
                res_even[(c as u8 - 'a' as u8) as usize] += 1;
            }
        }

        (res_odd, res_even)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ret = Solution::check_strings("abcdba".to_owned(), "cabdab".to_owned());
        assert_eq!(ret, true);
    }

    #[test]
    fn test_2() {
        let ret = Solution::check_strings("abe".to_owned(), "bea".to_owned());
        assert_eq!(ret, false);
    }

    #[test]
    fn test_2839_1() {
        let ret = Solution::check_strings("abcd".to_owned(), "cdab".to_owned());
        assert_eq!(ret, true);
    }

    #[test]
    fn test_2839_2() {
        let ret = Solution::check_strings("abcd".to_owned(), "dacb".to_owned());
        assert_eq!(ret, false);
    }

    #[test]
    fn test_az() {
        let ret = Solution::check_strings("aazz".to_owned(), "zzaa".to_owned());
        assert_eq!(ret, true);
    }

    #[test]
    fn test_single_1() {
        let ret = Solution::check_strings("a".to_owned(), "a".to_owned());
        assert_eq!(ret, true);
    }

    #[test]
    fn test_single_2() {
        let ret = Solution::check_strings("a".to_owned(), "z".to_owned());
        assert_eq!(ret, false);
    }
}
