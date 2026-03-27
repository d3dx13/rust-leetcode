#![allow(unused)]

use std::collections::HashSet;

struct Solution;

impl Solution {
    /// [3548. Equal Sum Grid Partition II](https://leetcode.com/problems/equal-sum-grid-partition-ii)
    ///
    /// You are given an m x n matrix grid of positive integers. Your task is to determine if it is possible to make either one horizontal or one vertical cut on the grid such that:
    /// - Each of the two resulting sections formed by the cut is non-empty.
    /// - The sum of elements in both sections is equal,
    /// or can be made equal by discounting at most one single cell in total (from either section).
    /// - If a cell is discounted, the rest of the section must remain connected.
    ///
    /// Return true if such a partition exists; otherwise, return false.
    ///
    /// Note: A section is connected if every cell in it can be reached from any other cell by moving up, down, left, or right through other cells in the section.
    ///
    pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
        // Separate processing for "corner cases", returning a result if the grid is a "corner case".
        // All code below this processing handles common scenarios
        // where the matrix is ​​not a 1x1 cell or a horizontal or vertical line.
        match Solution::process_lines(&grid) {
            Some(ret) => return ret,
            None => {}
        }

        let total_sum: i64 = grid.iter().flatten().map(|&x| x as i64).sum();

        // Check for a valid cut along the x-axis in the forward direction.
        if Solution::process_martix(&grid, total_sum) {
            return true;
        };

        // Check for a valid cut along the x-axis in the reversed direction.
        let mut grid = grid; // make grid mutable
        grid.reverse();
        if Solution::process_martix(&grid, total_sum) {
            return true;
        };

        // Check for a valid cut along the y-axis in the reversed direction.
        let mut grid_transposed: Vec<Vec<i32>> = Vec::with_capacity(grid[0].len());
        for x in 0..grid[0].len() {
            let mut line: Vec<i32> = Vec::with_capacity(grid.len());
            for y in 0..grid.len() {
                line.push(grid[y][x]);
            }
            grid_transposed.push(line);
        }
        if Solution::process_martix(&grid_transposed, total_sum) {
            return true;
        };

        // Check for a valid cut along the y-axis in the forward direction.
        grid_transposed.reverse();
        if Solution::process_martix(&grid_transposed, total_sum) {
            return true;
        };

        return false; // All checks failed.
    }

    /// Handle "corner cases" where m or n equals 1, that is, when the grid is a line or a single cell.
    /// This processing is faster and runs in Log(N) time.
    pub fn process_lines(grid: &Vec<Vec<i32>>) -> Option<bool> {
        if grid.len() < 2 && grid[0].len() < 2 {
            // println!("1 by 1");
            return Some(false);
        }
        if grid.len() > 1 && grid[0].len() > 1 {
            return None;
        }
        let last_index = grid.len() * grid[0].len() - 1;
        let cumulative_sum: Vec<i64> = grid
            .iter()
            .flatten()
            .scan(0i64, |sum, &x| {
                *sum += x as i64;
                Some(*sum)
            })
            .collect();

        // Check full array
        if cumulative_sum[last_index] % 2 == 0 {
            let target = cumulative_sum[last_index] / 2;
            match cumulative_sum[0..last_index].binary_search(&target) {
                Ok(_) => {
                    // println!("line full");
                    return Some(true);
                }
                Err(_) => {}
            }
        }

        // Check array without last element
        if cumulative_sum[last_index - 1] % 2 == 0 {
            let target = cumulative_sum[last_index - 1] / 2;
            match cumulative_sum[0..last_index - 1].binary_search(&target) {
                Ok(_) => {
                    // println!("line without last element");
                    return Some(true);
                }
                Err(_) => {}
            }
        }

        // Check array without first element
        if (cumulative_sum[last_index] - cumulative_sum[0]) % 2 == 0 {
            let target = (cumulative_sum[last_index] - cumulative_sum[0]) / 2 + cumulative_sum[0];
            match cumulative_sum[1..last_index].binary_search(&target) {
                Ok(_) => {
                    // println!("line without first element");
                    return Some(true);
                }
                Err(_) => {}
            }
        }

        // HotFix 1: Check array without n-th element in the middle, with cut among this element
        // vec: 2 5 20  3  4
        // cum: 2 7 27 30 34
        // for n in 1..=3 cut
        // vec[n] = cum[n] - cum[n-1]
        // after cut(n):
        //   left = cum[n-1]
        //   right = cum[last_index] - cum[n]
        // check if left==right is:
        // cum[n-1] ?= cum[last_index] - cum[n]
        // cum[n-1] + cum[n] ?= cum[last_index]
        for n in 1..last_index {
            if cumulative_sum[n - 1] + cumulative_sum[n] == cumulative_sum[last_index] {
                return Some(true);
            }
        }

        // println!("line impossible");
        return Some(false);
    }

    pub fn process_martix(grid: &Vec<Vec<i32>>, total_sum: i64) -> bool {
        let mut removable: HashSet<i32> = HashSet::new();
        removable.insert(0); // Handle case without removing values

        let mut cumulative_sum: i64 = 0;
        for y in 0..grid.len() {
            if y == 0 {
                removable.insert(grid[0][0]);
                removable.insert(grid[0][grid[0].len() - 1]);
            } else if y == 1 {
                removable.extend(grid[0].iter());
                removable.extend(grid[1].iter());
            } else {
                removable.extend(grid[y].iter());
            }
            cumulative_sum += grid[y].iter().map(|&x| x as i64).sum::<i64>();

            // left_part : cumulative_sum - removable
            // right_part : total_sum - cumulative_sum
            // left_part ?= right_part
            // cumulative_sum - removable ?= total_sum - cumulative_sum
            // cumulative_sum - removable ?= total_sum - cumulative_sum
            // removable ?= 2 * cumulative_sum - total_sum
            let desired_value_i64 = 2 * cumulative_sum - total_sum;
            if desired_value_i64 > i32::MAX as i64 || desired_value_i64 < i32::MIN as i64 {
                continue; // HotFix 2: Check for desired_value overflow
            }
            let desired_value = desired_value_i64 as i32;
            if removable.contains(&desired_value) {
                return true;
            }
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
        let ret = Solution::can_partition_grid(vec![vec![1, 2], vec![3, 4]]);
        assert_eq!(ret, true);
    }

    #[test]
    fn test_3() {
        let ret = Solution::can_partition_grid(vec![vec![1, 2, 4], vec![2, 3, 5]]);
        assert_eq!(ret, false);
    }

    #[test]
    fn test_4() {
        let ret = Solution::can_partition_grid(vec![vec![4, 1, 8], vec![3, 2, 6]]);
        assert_eq!(ret, false);
    }

    #[test]
    fn test_937() {
        let ret = Solution::can_partition_grid(vec![vec![100000], vec![86218], vec![100000]]);
        assert_eq!(ret, true);
    }

    #[test]
    fn test_940() {
        let mut line1 = vec![100000; 42950];
        line1.push(10247);
        let ret = Solution::can_partition_grid(vec![line1, vec![1; 42951]]);
        assert_eq!(ret, false);
    }

    #[test]
    fn test_square() {
        let ret = Solution::can_partition_grid(vec![
            vec![0 + 1, 0 + 2, 0 + 3, 0 + 4, 0 + 5],
            vec![5 + 1, 5 + 2, 5 + 3, 5 + 4, 5 + 5],
            vec![10 + 1, 10 + 2, 10 + 3, 10 + 4, 10 + 5],
            vec![15 + 1, 15 + 2, 15 + 3, 15 + 4, 15 + 5],
            vec![20 + 1, 20 + 2, 20 + 3, 20 + 4, 20 + 5],
        ]);
        assert_eq!(ret, false);
    }

    #[test]
    fn test_horizontal_line_1by1() {
        let ret = Solution::can_partition_grid(vec![vec![1]]);
        assert_eq!(ret, false);
    }

    #[test]
    fn test_horizontal_line_1by2_equal() {
        let ret = Solution::can_partition_grid(vec![vec![2, 2]]);
        assert_eq!(ret, true);
    }

    #[test]
    fn test_horizontal_line_1by2_unequal() {
        let ret = Solution::can_partition_grid(vec![vec![6, 7]]);
        assert_eq!(ret, false);
    }

    #[test]
    fn test_horizontal_line_2by1_equal() {
        let ret = Solution::can_partition_grid(vec![vec![2], vec![2]]);
        assert_eq!(ret, true);
    }

    #[test]
    fn test_horizontal_line_2by1_unequal() {
        let ret = Solution::can_partition_grid(vec![vec![6], vec![7]]);
        assert_eq!(ret, false);
    }

    #[test]
    fn test_horizontal_line_r100() {
        let ret = Solution::can_partition_grid(vec![vec![1, 2, 3, 100]]);
        assert_eq!(ret, true);
    }

    #[test]
    fn test_horizontal_line_l100() {
        let ret = Solution::can_partition_grid(vec![vec![100, 1, 2, 3]]);
        assert_eq!(ret, true);
    }

    #[test]
    fn test_horizontal_line_1221() {
        let ret = Solution::can_partition_grid(vec![vec![1, 2, 2, 1]]);
        assert_eq!(ret, true);
    }

    #[test]
    fn test_horizontal_line_mid20() {
        let ret = Solution::can_partition_grid(vec![vec![1, 2, 20, 2, 1]]);
        assert_eq!(ret, true);
    }
}
