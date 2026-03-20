#![allow(unused)]
struct Solution;

impl Solution {
    /// [3567. Minimum Absolute Difference in Sliding Submatrix](https://leetcode.com/problems/minimum-absolute-difference-in-sliding-submatrix/description/)
    ///
    /// You are given an m x n integer matrix grid and an integer k.
    ///
    /// For every contiguous k x k submatrix of grid, compute the minimum absolute difference between any two distinct values within that submatrix.
    ///
    /// Return a 2D array ans of size (m - k + 1) x (n - k + 1), where ans[i][j] is the minimum absolute difference in the submatrix whose top-left corner is (i, j) in grid.
    ///
    /// Note: If all elements in the submatrix have the same value, the answer will be 0.
    ///
    /// A submatrix (x1, y1, x2, y2) is a matrix that is formed by choosing all cells matrix[x][y] where x1 <= x <= x2 and y1 <= y <= y2.
    ///
    pub fn min_abs_diff(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        // Change types of input parameters
        let m = grid.len();
        let n = grid[0].len();
        let k = k as usize;

        // Precreate result
        let mut res = vec![vec![0; (n - k + 1) as usize]; (m - k + 1) as usize];

        // Corner case
        if k == 1 {
            return res;
        }

        // Feature 1:
        let mut kgrid = Vec::with_capacity(k * k);

        // Loop through all k by k submatrix
        for i in 0..=m - k {
            for j in 0..=n - k {
                // Feature 1:
                kgrid.clear();

                // Feature 1:
                for x in i..i + k {
                    for y in j..j + k {
                        kgrid.push(grid[x][y]);
                    }
                }

                // Feature 2:
                // Unstable sorting (faster than regular sort because it doesn't need to maintain the relative order of equal elements)
                kgrid.sort_unstable();

                // Feature 3:
                // In-place deduplication (must be used after sorting; compresses adjacent identical elements into one).
                kgrid.dedup();

                // kgrid is occupied by a single number.
                if kgrid.len() < 2 {
                    res[i][j] = 0;
                } else {
                    let mut kmin = i32::MAX;
                    for pair in kgrid.windows(2) {
                        kmin = kmin.min(pair[1] - pair[0]);
                    }
                    res[i][j] = kmin;
                }
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
        let ret = Solution::min_abs_diff(vec![vec![1, 8], vec![3, -2]], 2);
        assert_eq!(ret, vec![vec![2]]);
    }

    #[test]
    fn test_2() {
        let ret = Solution::min_abs_diff(vec![vec![3, -1]], 1);
        assert_eq!(ret, vec![vec![0, 0]]);
    }

    #[test]
    fn test_3() {
        let ret = Solution::min_abs_diff(vec![vec![1, -2, 3], vec![2, 3, 5]], 2);
        assert_eq!(ret, vec![vec![1, 2]]);
    }

    #[test]
    fn test_722() {
        let ret = Solution::min_abs_diff(
            vec![
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            ],
            5,
        );
        assert_eq!(
            ret,
            vec![
                vec![0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0],
            ],
        );
    }
}
