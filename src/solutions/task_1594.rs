#![allow(unused)]

struct Solution;

impl Solution {
    /// [1594. Maximum Non Negative Product in a Matrix](https://leetcode.com/problems/maximum-non-negative-product-in-a-matrix)
    ///
    /// You are given a m x n matrix grid. Initially, you are located at the top-left corner (0, 0), and in each step, you can only move right or down in the matrix.
    ///
    /// Among all possible paths starting from the top-left corner (0, 0) and ending in the bottom-right corner (m - 1, n - 1), find the path with the maximum non-negative product. The product of a path is the product of all integers in the grid cells visited along the path.
    ///
    /// Return the maximum non-negative product modulo 109 + 7. If the maximum product is negative, return -1.
    ///
    /// Notice that the modulo is performed after getting the maximum product.
    ///
    pub fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
        // Prepare input values
        let y_size = grid.len();
        let x_size = grid[0].len();

        // (min, max) for path from (0, 0) to (y, x)
        let mut memory = vec![vec![(i64::MAX, i64::MIN); x_size]; y_size];

        // Fill left and top borders
        memory[0][0] = (grid[0][0] as i64, grid[0][0] as i64);
        for y in 1..y_size {
            memory[y][0].0 = memory[y - 1][0].0 * grid[y][0] as i64;
            memory[y][0].1 = memory[y - 1][0].1 * grid[y][0] as i64;
        }
        for x in 1..x_size {
            memory[0][x].0 = memory[0][x - 1].0 * grid[0][x] as i64;
            memory[0][x].1 = memory[0][x - 1].1 * grid[0][x] as i64;
        }

        for y in 1..y_size {
            for x in 1..x_size {
                // Optimization 1: check for zero product
                if grid[y][x] == 0 {
                    memory[y][x] = (0, 0);
                    continue;
                }

                let value = grid[y][x] as i64;
                let neighbours = (
                    memory[y - 1][x].0.min(memory[y][x - 1].0),
                    memory[y - 1][x].1.max(memory[y][x - 1].1),
                ); // (min, max)
                if value > 0 {
                    memory[y][x].0 = neighbours.0 * value;
                    memory[y][x].1 = neighbours.1 * value;
                } else {
                    memory[y][x].0 = neighbours.1 * value;
                    memory[y][x].1 = neighbours.0 * value;
                }
            }
        }
        // dbg!(memory);
        // todo!();

        let result = memory[grid.len() - 1][grid[0].len() - 1].1; // maximum
        // let result = Solution::check_paths(&grid, 1, 0, 0);
        // dbg!(result);
        // todo!();

        if result < 0 {
            return -1;
        } else {
            return (result % 1_000_000_007) as i32;
        }
    }

    fn check_paths_to(grid: &Vec<Vec<i32>>, y: usize, x: usize) -> i64 {
        todo!()
    }

    fn check_paths(grid: &Vec<Vec<i32>>, product: i64, y: usize, x: usize) -> i64 {
        let is_y_in_border = y == grid.len() - 1;
        let is_x_in_border = x == grid[0].len() - 1;
        let product = product * grid[y][x] as i64;

        let result;
        if product == 0 {
            // Optimization 1: check for zero product
            result = 0;
            // dbg!(result);
        } else if is_y_in_border && is_x_in_border {
            result = product;
            // dbg!(result);
        } else if is_y_in_border {
            result = Solution::check_paths(grid, product, y, x + 1);
        } else if is_x_in_border {
            result = Solution::check_paths(grid, product, y + 1, x);
        } else {
            result = Solution::check_paths(grid, product, y, x + 1).max(Solution::check_paths(
                grid,
                product,
                y + 1,
                x,
            ));
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ret =
            Solution::max_product_path(vec![vec![-1, -2, -3], vec![-2, -3, -3], vec![-3, -3, -2]]);
        assert_eq!(ret, -1);
    }

    #[test]
    fn test_2() {
        let ret = Solution::max_product_path(vec![vec![1, -2, 1], vec![1, -2, 1], vec![3, -4, 1]]);
        assert_eq!(ret, 8);
    }

    #[test]
    fn test_3() {
        let ret = Solution::max_product_path(vec![vec![1, 3], vec![0, -4]]);
        assert_eq!(ret, 0);
    }

    #[test]
    fn test_138() {
        let ret = Solution::max_product_path(vec![
            vec![1, -1, 2, 1, -1, 0, 0, 4, 3, 2, 0, -2, -2],
            vec![-2, 3, 3, -1, -1, 0, 0, -2, 4, -3, 3, 0, 0],
            vec![-4, -1, -1, -2, 2, -1, -2, -2, 0, 3, -1, -4, 1],
            vec![-3, 4, -3, 0, -3, 1, -3, 1, 4, 4, -4, -4, -2],
            vec![3, -3, 1, 0, -1, -4, -4, -4, 3, 2, 2, 3, 3],
            vec![2, -1, -1, -4, -3, -3, 4, 2, 3, 4, 4, -4, 0],
            vec![4, -1, 2, -3, -1, -1, -3, -4, 4, 4, 4, -3, -1],
            vec![-3, -4, 4, -2, -1, 2, 3, -1, 2, 3, 4, 4, -4],
            vec![-3, -1, -2, 1, 1, -1, -3, -4, -3, 1, -3, 3, -4],
            vec![2, 4, 4, 4, -3, -3, 1, -1, 3, 4, -1, 1, 4],
            vec![2, -2, 0, 4, -1, 0, -2, 4, -4, 0, 0, 2, -3],
            vec![1, 1, -3, 0, -4, -4, -4, -4, 0, -1, -4, -1, 0],
            vec![3, -1, -3, -3, -3, -2, -1, 4, -1, -2, 4, 2, 3],
        ]);
        assert_eq!(ret, 459630706);
    }

    #[test]
    fn test_153() {
        let ret = Solution::max_product_path(vec![
            vec![2, -3, 4, -1, 3, -1, 4, 3, -4, 2, 0, 2, -1, 4, 3],
            vec![2, -2, -3, 1, 3, 0, -4, -2, 0, 0, -1, -4, 1, -3, -2],
            vec![-2, 0, 1, -2, -2, -2, 1, 0, 3, 1, -2, 2, -3, 0, -3],
            vec![0, -4, 2, 0, -1, -2, 3, -4, 0, 4, -2, -4, 2, -1, 2],
            vec![0, -3, 3, 3, 4, -3, -2, 4, -1, 4, -2, 3, 3, 0, -1],
            vec![3, -1, -4, 1, -4, -1, 2, 4, 4, 2, 1, -1, -4, 3, -1],
            vec![1, -1, 2, -1, 4, 2, -1, 0, 4, 2, 4, 2, -2, 0, -3],
            vec![-4, -1, 1, 3, -1, -2, -1, -2, -2, 2, -2, -3, -4, 2, 0],
            vec![-1, -1, -2, 0, -4, 0, -4, -4, -2, -2, -4, -2, -4, 1, 2],
            vec![1, -1, -1, 3, 0, 3, 4, 0, 3, -1, 0, -1, 4, -4, 2],
            vec![-3, 2, -2, -1, 3, 4, -1, -2, 2, 4, 3, 4, 2, 2, 4],
            vec![1, -2, -4, -4, 1, -2, 1, -3, 4, 4, -1, 2, -3, -3, 3],
            vec![-2, 2, 0, 3, -2, 0, -1, -4, -4, -2, -4, 3, -3, -2, -2],
            vec![2, 1, 1, 3, -4, -2, 0, 2, 1, 2, 2, 3, -1, 3, 3],
            vec![-4, -1, 0, -3, 4, 1, 1, 4, -4, -1, 1, 4, 4, -3, -4],
        ]);
        assert_eq!(ret, 560463606);
    }

    #[test]
    fn test_155() {
        let ret = Solution::max_product_path(vec![
            vec![0, -1, -2, -4, 2, 1, 1, 0, 4, 3, -2, -1, 1, -2, 3],
            vec![2, 2, -4, 0, -4, -2, -4, 0, 4, -4, -2, -4, -1, 1, -4],
            vec![-3, -1, 0, -2, 1, 4, -1, -1, -4, -1, -3, -4, 2, 2, -4],
            vec![-4, -4, -2, 1, -1, 0, -2, 4, 0, 1, -4, 0, 4, 4, -4],
            vec![-4, 2, -3, -1, -2, 0, 1, 2, -2, -3, -4, 2, -2, -4, 0],
            vec![2, -3, 3, -1, -2, 4, -3, 4, 0, 2, 0, 4, -3, -4, 0],
            vec![2, 0, -2, -2, -1, -2, -2, 3, -2, 2, -3, 4, 0, 1, 4],
            vec![3, 1, 2, 2, 1, -3, 1, 4, -4, -3, 4, -4, 1, 1, 3],
            vec![-3, -4, -1, 3, 4, -2, -2, 0, 0, 1, -1, 3, -2, -3, 1],
            vec![1, 2, 4, -1, 0, 0, -3, 1, -1, -4, 1, 3, 0, 4, 4],
            vec![-3, -1, -2, 0, 1, 1, -2, 0, 4, -4, 0, 4, -3, 1, -1],
            vec![2, -2, 2, 1, 0, 2, -2, 3, -4, 2, -2, 1, -3, 1, -3],
            vec![3, -2, 0, -1, -3, -1, -3, -2, 3, 1, -1, 2, 3, -4, -2],
            vec![1, 0, -1, -4, 2, -1, 2, 1, 2, -3, 2, 0, -1, -3, 1],
            vec![4, -2, 1, 3, 3, 3, 0, 2, -4, 1, 2, 4, -2, 1, -4],
        ]);
        assert_eq!(ret, 0);
    }
}
