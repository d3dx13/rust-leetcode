#![allow(unused)]

struct Solution;

impl Solution {
    /// [2906. Construct Product Matrix](https://leetcode.com/problems/construct-product-matrix)
    ///
    /// Given a 0-indexed 2D integer matrix grid of size n * m, we define a 0-indexed 2D matrix p of size n * m as the product matrix of grid if the following condition is met:
    ///
    /// - Each element p[i][j] is calculated as the product of all elements in grid except for the element grid[i][j]. This product is then taken modulo 12345.
    ///
    /// Return the product matrix of grid.
    ///
    pub fn construct_product_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let modulo: i32 = 12345;
        let y_size = grid.len();
        let x_size = grid[0].len();
        let grid: Vec<i32> = grid.into_iter().flatten().map(|x| x % modulo).collect();
        let grid_len = grid.len();
        // dbg!(grid);
        // todo!();

        // IF: a * b === c
        // a === a_d * R + a_r
        // a % R === a_r
        //
        // (a_d * R + a_r) * (b_d * R + b_r) === c_d * R + c_r
        // => (a_r * b_r) % R = c_r
        // We should not store the products of numbers, but the remainder of the product
        //
        // Let's keep the remainder of the product of all the numbers to the left and right of the number
        //
        // Potential optimization 1: If a sequence contains a number with reminder = 0,
        // any product containing it will be evenly divisible by itself with a remainder of zero.
        // If there is only one such number in the sequence, the entire result will consist of zeros except for that one.
        // If there are two or more such numbers, the entire array will consist of zeros.
        let mut product_left: Vec<i32> = Vec::with_capacity(y_size * x_size);
        let mut product_right: Vec<i32> = Vec::with_capacity(y_size * x_size);
        product_left.push(grid[0] % modulo);
        for i in 1..grid_len {
            product_left.push((product_left[i - 1] * grid[i]) % modulo);
        }
        product_right.push(grid[grid_len - 1] % modulo);
        for i in 1..grid_len {
            product_right.push((product_right[i - 1] * grid[grid_len - i - 1]) % modulo);
        }
        product_right.reverse();
        // dbg!(grid.clone());
        // dbg!(product_left.clone());
        // dbg!(product_right.clone());
        // todo!();

        let mut res = Vec::with_capacity(y_size);
        for y in 0..y_size {
            let mut line = Vec::with_capacity(x_size);
            for x in 0..x_size {
                let i = y * x_size + x;
                let mut p = 1;
                if i > 0 {
                    p = (p * product_left[i - 1]) % modulo;
                }
                if i < grid_len - 1 {
                    p = (p * product_right[i + 1]) % modulo;
                }
                line.push(p);
            }
            res.push(line);
        }
        // dbg!(res.clone());
        // todo!();

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ret = Solution::construct_product_matrix(vec![vec![1, 2], vec![3, 4]]);
        assert_eq!(ret, vec![vec![24, 12], vec![8, 6]]);
    }

    #[test]
    fn test_2() {
        let ret = Solution::construct_product_matrix(vec![vec![12345], vec![2], vec![1]]);
        assert_eq!(ret, vec![vec![2], vec![0], vec![0]]);
    }

    #[test]
    fn test_1268() {
        let ret = Solution::construct_product_matrix(vec![
            vec![414750857],
            vec![449145368],
            vec![767292749],
        ]);
        assert_eq!(ret, vec![vec![1462], vec![3103], vec![9436]]);
    }
}
