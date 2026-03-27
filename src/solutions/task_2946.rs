#![allow(unused)]

struct Solution;

impl Solution {
    /// [2946. Matrix Similarity After Cyclic Shifts](https://leetcode.com/problems/matrix-similarity-after-cyclic-shifts/)
    ///
    /// You are given an m x n integer matrix mat and an integer k. The matrix rows are 0-indexed.
    ///
    /// The following proccess happens k times:
    /// - Even-indexed rows (0, 2, 4, ...) are cyclically shifted to the left.
    /// - Odd-indexed rows (1, 3, 5, ...) are cyclically shifted to the right.
    ///
    /// Return true if the final modified matrix after k steps is identical to the original matrix, and false otherwise.
    ///
    pub fn are_similar(mat: Vec<Vec<i32>>, k: i32) -> bool {
        let y_size = mat.len();
        let x_size = mat[0].len() as i32;
        let k = k % x_size as i32;

        for y in 0..y_size {
            for x in 0..x_size {
                let direction = { if x % 2 == 0 { 1 } else { -1 } };
                let shifted_x = (x_size + x + k * direction) % x_size;
                if mat[y][x as usize] != mat[y][shifted_x as usize] {
                    return false;
                }
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
        let ret = Solution::are_similar(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 4);
        assert_eq!(ret, false);
    }

    #[test]
    fn test_2() {
        let ret = Solution::are_similar(
            vec![vec![1, 2, 1, 2], vec![5, 5, 5, 5], vec![6, 3, 6, 3]],
            2,
        );
        assert_eq!(ret, true);
    }

    #[test]
    fn test_3() {
        let ret = Solution::are_similar(vec![vec![2, 2], vec![2, 2]], 3);
        assert_eq!(ret, true);
    }

    #[test]
    fn test_inplace_max_minus_one() {
        let mut mat: Vec<Vec<i32>> = Vec::with_capacity(24);
        for i in 0..24 {
            mat.push(((i * 24 + 1)..=(i * 24 + 24)).collect());
        }
        let ret = Solution::are_similar(mat, 48);
        assert_eq!(ret, true);
    }

    #[test]
    fn test_inplace_max() {
        let mut mat: Vec<Vec<i32>> = Vec::with_capacity(25);
        for i in 0..25 {
            mat.push(((i * 25 + 1)..=(i * 25 + 25)).collect());
        }
        let ret = Solution::are_similar(mat, 50);
        assert_eq!(ret, true);
    }
}
