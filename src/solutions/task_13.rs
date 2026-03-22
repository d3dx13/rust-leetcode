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
        let mut res = 0;
        let mut last_char = '\0';

        for c in s.chars() {
            match c {
                'I' => res += 1,
                'V' => {
                    if last_char == 'I' {
                        res += 3 // 5 - 1 - 1
                    } else {
                        res += 5
                    }
                }
                'X' => {
                    if last_char == 'I' {
                        res += 8 // 10 - 1 - 1
                    } else {
                        res += 10
                    }
                }
                'L' => {
                    if last_char == 'X' {
                        res += 30 // 50 - 10 - 10
                    } else {
                        res += 50
                    }
                }
                'C' => {
                    if last_char == 'X' {
                        res += 80 // 100 - 10 - 10
                    } else {
                        res += 100
                    }
                }
                'D' => {
                    if last_char == 'C' {
                        res += 300 // 500 - 100 - 100
                    } else {
                        res += 500
                    }
                }
                'M' => {
                    if last_char == 'C' {
                        res += 800 // 1000 - 100 - 100
                    } else {
                        res += 1000
                    }
                }
                _ => panic!("Illegal character: {}", c),
            }
            last_char = c;
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
