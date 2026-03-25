#![allow(unused)]

struct Solution;

impl Solution {
    /// [3546. Equal Sum Grid Partition I](https://leetcode.com/problems/equal-sum-grid-partition-i)
    ///
    /// You are given an m x n matrix grid of positive integers. Your task is to determine if it is possible to make either one horizontal or one vertical cut on the grid such that:
    /// - Each of the two resulting sections formed by the cut is non-empty.
    /// - The sum of the elements in both sections is equal.
    ///
    /// Return true if such a partition exists; otherwise return false.
    ///
    pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
        // Get grid shape
        let y_size = grid.len();
        let x_size = grid[0].len();

        // Filll cumulative sum arrays
        let mut y_sums: Vec<i64> = Vec::with_capacity(y_size);
        let mut x_sums: Vec<i64> = Vec::with_capacity(x_size);
        y_sums.push(grid[0].iter().map(|&x| x as i64).sum());
        for y in 1..y_size {
            let sum: i64 = grid[y].iter().map(|&x| x as i64).sum();
            y_sums.push(sum + y_sums[y - 1]);
        }
        for x in 0..x_size {
            let mut sum: i64 = grid[0][x] as i64;
            for y in 1..y_size {
                sum += grid[y][x] as i64;
            }
            if x == 0 {
                x_sums.push(sum);
            } else {
                x_sums.push(sum + x_sums[x - 1]);
            }
        }
        // dbg!(y_sums.clone());
        // dbg!(x_sums.clone());

        // Optimization 1:
        // let last_sum = y_sums[y_size - 1] = x_sums[x_size - 1]
        // If the number is even, then such vertical or horizontal cut potentially exists.
        // If the number is odd, then such cut does not exist.
        // let target_sum = last_sum / 2
        // assert_eq!(y_sums[y_size - 1], x_sums[x_size - 1]);
        if y_sums[y_size - 1] % 2 != 0 {
            return false;
        }
        let target_sum: i64 = y_sums[y_size - 1] / 2;

        // Optimization 2: Binary Search approach
        x_sums.pop();
        match x_sums.binary_search(&target_sum) {
            Ok(_) => return true,
            Err(_) => {}
        }
        y_sums.pop();
        match y_sums.binary_search(&target_sum) {
            Ok(_) => return true,
            Err(_) => {}
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ret = Solution::can_partition_grid(vec![vec![1, 4], vec![2, 3]]);
        assert_eq!(ret, true);
    }

    #[test]
    fn test_2() {
        let ret = Solution::can_partition_grid(vec![vec![1, 3], vec![2, 4]]);
        assert_eq!(ret, false);
    }

    #[test]
    fn test_zero() {
        let ret = Solution::can_partition_grid(vec![vec![0, 0], vec![0, 0]]);
        assert_eq!(ret, true);
    }

    #[test]
    fn test_zero_thin() {
        let ret = Solution::can_partition_grid(vec![vec![0]]);
        assert_eq!(ret, false);
    }

    #[test]
    fn test_six_seven() {
        let ret = Solution::can_partition_grid(vec![vec![6, 7], vec![6, 7]]);
        assert_eq!(ret, true);
    }

    #[test]
    fn test_six_seven_thin() {
        let ret = Solution::can_partition_grid(vec![vec![6], vec![7], vec![6], vec![7]]);
        assert_eq!(ret, true);
    }

    // #[test]
    fn test_overflow() {
        let ret = Solution::can_partition_grid(vec![vec![100 * 1000; 1000]; 100]);
        assert_eq!(ret, true);
    }
}
