#![allow(unused)]
struct Solution;

impl Solution {
    /// [3643. Flip Square Submatrix Vertically](https://leetcode.com/problems/flip-square-submatrix-vertically/)
    ///
    /// You are given an m x n integer matrix grid, and three integers x, y, and k.
    ///
    /// The integers x and y represent the row and column indices of the top-left corner of a square submatrix and the integer k represents the size (side length) of the square submatrix.
    ///
    /// Your task is to flip the submatrix by reversing the order of its rows vertically.
    ///
    /// Return the updated matrix.
    ///
    pub fn reverse_submatrix(grid: Vec<Vec<i32>>, x: i32, y: i32, k: i32) -> Vec<Vec<i32>> {
        // Rename input parameters
        let height = grid.len();
        let width = grid[0].len();
        let square_y = x as usize;
        let square_x = y as usize;
        let square_size = k as usize;

        // Create result array with desired rules
        let mut res: Vec<Vec<i32>> = Vec::with_capacity(height);
        for y in 0..height {
            let is_y_inside_square = (y >= square_y) && (y < (square_y + square_size));
            let mut row = Vec::with_capacity(width);
            for x in 0..width {
                let is_x_inside_square = (x >= square_x) && (x < (square_x + square_size));
                if is_y_inside_square && is_x_inside_square {
                    let target_y = (square_y + square_size - 1) - (y - square_y);
                    row.push(grid[target_y][x]);
                } else {
                    row.push(grid[y][x]);
                }
            }
            res.push(row);
        }

        // dbg!(res.clone());

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ret = Solution::reverse_submatrix(
            vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12],
                vec![13, 14, 15, 16],
            ],
            1,
            0,
            3,
        );
        assert_eq!(
            ret,
            vec![
                vec![1, 2, 3, 4],
                vec![13, 14, 15, 8],
                vec![9, 10, 11, 12],
                vec![5, 6, 7, 16],
            ],
        );
    }

    #[test]
    fn test_2() {
        let ret = Solution::reverse_submatrix(vec![vec![3, 4, 2, 3], vec![2, 3, 4, 2]], 0, 2, 2);
        assert_eq!(ret, vec![vec![3, 4, 4, 2], vec![2, 3, 2, 3],],);
    }

    #[test]
    fn test_center() {
        let ret = Solution::reverse_submatrix(
            vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12],
                vec![13, 14, 15, 16],
            ],
            1,
            1,
            2,
        );
        assert_eq!(
            ret,
            vec![
                vec![1, 2, 3, 4],
                vec![5, 10, 11, 8],
                vec![9, 6, 7, 12],
                vec![13, 14, 15, 16],
            ],
        );
    }
}
