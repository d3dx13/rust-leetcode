#![allow(unused)]

struct Solution;

impl Solution {
    /// [2573. Find the String with LCP](https://leetcode.com/problems/find-the-string-with-lcp)
    ///
    /// We define the lcp matrix of any 0-indexed string word of n lowercase English letters as an n x n grid such that:
    /// - lcp[i][j] is equal to the length of the longest common prefix between the substrings word[i,n-1] and word[j,n-1].
    ///
    /// Given an n x n matrix lcp, return the alphabetically smallest string word that corresponds to lcp.
    /// If there is no such string, return an empty string.
    ///
    /// A string a is lexicographically smaller than a string b (of the same length) if in the first position where a and b differ,
    /// string a has a letter that appears earlier in the alphabet than the corresponding letter in b.
    /// For example, "aabd" is lexicographically smaller than "aaca" because the first position they differ is at the third letter, and 'b' comes before 'c'.
    ///
    pub fn find_the_string(lcp: Vec<Vec<i32>>) -> String {
        if !Solution::is_lcp_valid(&lcp) {
            return String::new();
        }

        // word calculation from lcp
        let n = lcp.len();
        let mut word = vec!['a'; n];
        for y in 0..n {
            for x in 0..n {
                if lcp[y][x] == 0 && word[y] == word[x] {
                    word[x] = (word[x] as u8 + 1) as char;
                    if word[x] > 'z' {
                        return String::new();
                    }
                }
            }
        }

        // lcp verification with word
        for y in 0..n {
            for x in 0..n {
                if word[y] == word[x] {
                    if y == n - 1 || x == n - 1 {
                        if lcp[y][x] != 1 {
                            return String::new();
                        }
                    } else {
                        if lcp[y][x] != lcp[y + 1][x + 1] + 1 {
                            return String::new();
                        }
                    }
                } else {
                    if lcp[y][x] != 0 {
                        return String::new();
                    }
                }
            }
        }

        // dbg!(word);
        // todo!();

        String::from_iter(word)
    }

    pub fn is_lcp_valid(lcp: &Vec<Vec<i32>>) -> bool {
        // O(1): Check for non-sqare matrix
        if lcp.len() != lcp[0].len() {
            return false;
        }

        let n = lcp.len(); // Express n for clarity

        // O(n): Check that the diagonal of the square must correspond to the decreasing length
        for i in 0..n {
            if lcp[i][i] != (n - i) as i32 {
                return false;
            }
        }

        // O(n^2): Check that the matrix is ​​symmetrical
        // and its elements do not exceed the length of the maximum prefix on the diagonal
        for y in 1..n {
            for x in 0..y {
                if lcp[y][x] != lcp[x][y] {
                    return false;
                }
                if lcp[y][x] > lcp[y][y] {
                    return false;
                }
            }
        }

        // All checks have been passed
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let lcp = vec![
            vec![4, 0, 2, 0],
            vec![0, 3, 0, 1],
            vec![2, 0, 2, 0],
            vec![0, 1, 0, 1],
        ];
        let ret = Solution::find_the_string(lcp);

        assert_eq!(ret, "abab");
    }

    #[test]
    fn test_2() {
        let lcp = vec![
            vec![4, 3, 2, 1],
            vec![3, 3, 2, 1],
            vec![2, 2, 2, 1],
            vec![1, 1, 1, 1],
        ];
        let ret = Solution::find_the_string(lcp);
        assert_eq!(ret, "aaaa");
    }

    #[test]
    fn test_3() {
        let lcp = vec![
            vec![4, 3, 2, 1],
            vec![3, 3, 2, 1],
            vec![2, 2, 2, 1],
            vec![1, 1, 1, 3],
        ];
        let ret = Solution::find_the_string(lcp);
        assert_eq!(ret, "");
    }

    #[test]
    fn test_46() {
        let lcp = vec![
            vec![4, 1, 1, 1],
            vec![1, 3, 1, 1],
            vec![1, 1, 2, 1],
            vec![1, 1, 1, 1],
        ];
        let ret = Solution::find_the_string(lcp);
        assert_eq!(ret, "");
    }

    #[test]
    fn test_29() {
        let lcp = vec![
            vec![8, 0, 0, 0, 0, 1, 2, 0],
            vec![0, 7, 0, 1, 1, 0, 0, 1],
            vec![0, 0, 6, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 5, 1, 0, 0, 1],
            vec![0, 1, 0, 1, 4, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 3, 1, 0],
            vec![2, 0, 0, 0, 0, 1, 2, 0],
            vec![0, 1, 0, 1, 1, 0, 0, 1],
        ];
        let ret = Solution::find_the_string(lcp);
        assert_eq!(ret, "abcbbaab");
    }
}
