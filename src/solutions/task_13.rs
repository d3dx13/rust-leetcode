#![allow(unused)]

struct Solution;

impl Solution {
    /// [13. Roman to Integer](https://leetcode.com/problems/roman-to-integer)
    ///
    /// Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.
    ///
    /// Symbol       Value
    /// I             1
    /// V             5
    /// X             10
    /// L             50
    /// C             100
    /// D             500
    /// M             1000
    /// For example, 2 is written as II in Roman numeral, just two ones added together. 12 is written as XII, which is simply X + II. The number 27 is written as XXVII, which is XX + V + II.
    ///
    /// Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not IIII. Instead, the number four is written as IV. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:
    ///
    /// I can be placed before V (5) and X (10) to make 4 and 9.
    /// X can be placed before L (50) and C (100) to make 40 and 90.
    /// C can be placed before D (500) and M (1000) to make 400 and 900.
    /// Given a roman numeral, convert it to an integer.
    ///
    pub fn roman_to_int(s: String) -> i32 {
        let numbers: Vec<i32> = s
            .chars()
            .map(|c| match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => panic!("Illegal character: {}", c),
            })
            .collect();

        let mut res = 0;
        for i in 0..numbers.len() {
            if i < numbers.len() - 1 && numbers[i] < numbers[i + 1] {
                res -= numbers[i];
            } else {
                res += numbers[i];
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::roman_to_int("III".to_owned()), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::roman_to_int("LVIII".to_owned()), 58);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_owned()), 1994);
    }
}
