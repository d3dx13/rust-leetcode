#![allow(unused)]

/// # Intuition
///
/// The limitation on the value in the matrix from 0 to 1 and the maximum size of 10 by 10 makes one think that each row of the matrix can be represented as a 10-bit number.
///
/// # Approach
///
/// Extending this idea, we can use a 128-bit integer and losslessly compress the matrix into a single 128-bit number. Technically, this could store square matrices up to 11 by 11.
///
/// Now all that remains is to learn how to compress not just one matrix, but also all three possible rotations. This can be done without rotating the original matrix (which would require accessing it in a slow heap), simply by changing the indices.
///
/// Since we don't need to compare 4 rotations with 4 rotations, but just compare the original matrix with 4 others, we introduce the zero_rotation_only flag, which turns off the calculation of the other 3 rotations of the matrix.
///
/// # Complexity
/// - Time complexity: $$O(n^2)$$
///
/// We perform Zero rotations and Zero write operations to the matrices
///
/// - Space complexity: $$O(1)$$
///
struct Solution;

impl Solution {
    /// [1886. Determine Whether Matrix Can Be Obtained By Rotation](https://leetcode.com/problems/determine-whether-matrix-can-be-obtained-by-rotation)
    ///
    /// Given two n x n binary matrices mat and target, return true if it is possible to make mat equal to target by rotating mat in 90-degree increments, or false otherwise.
    ///
    pub fn find_rotation(mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        let mat = Solution::compress(mat, true);
        let target = Solution::compress(target, false);

        // dbg!(mat);
        // dbg!(target);
        // todo!();

        if mat.0 == target.0 || mat.0 == target.1 || mat.0 == target.2 || mat.0 == target.3 {
            return true;
        } else {
            return false;
        }
    }

    /// Returns an integer represintaions of bit matrix rotated by (0, 90, 180, 270) degrees
    ///
    /// **Arguments:**
    ///
    /// * `mat`: n by n (max n is 11) matrix with 0 or 1 values.
    fn compress(mat: Vec<Vec<i32>>, zero_rotation_only: bool) -> (u128, u128, u128, u128) {
        assert_eq!(mat.len(), mat[0].len());
        let n = mat.len();

        let mut res: (u128, u128, u128, u128) = (0, 0, 0, 0);

        for y in 0..n {
            for x in 0..n {
                if mat[y][x] > 0 {
                    res.0 += (1 << x) << (y * n);

                    if !zero_rotation_only {
                        let rot90_y = n - x - 1;
                        let rot90_x = y;
                        res.1 += (1 << rot90_x) << (rot90_y * n);

                        let rot180_y = n - y - 1;
                        let rot180_x = n - x - 1;
                        res.2 += (1 << rot180_x) << (rot180_y * n);

                        let rot270_y = x;
                        let rot270_x = n - y - 1;
                        res.3 += (1 << rot270_x) << (rot270_y * n);
                    }
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
    fn test_symmetrical() {
        let mat: (u128, u128, u128, u128) = Solution::compress(
            vec![
                vec![1, 0, 1, 1, 1],
                vec![1, 1, 0, 1, 0],
                vec![1, 0, 1, 0, 1],
                vec![0, 1, 0, 1, 1],
                vec![1, 1, 1, 0, 1],
            ],
            false,
        );
        // dbg!(mat);
        // todo!("REMOVE");
        assert_eq!(mat.0, mat.1);
        assert_eq!(mat.0, mat.2);
        assert_eq!(mat.0, mat.3);
    }

    #[test]
    fn test_1() {
        let ret =
            Solution::find_rotation(vec![vec![0, 1], vec![1, 0]], vec![vec![1, 0], vec![0, 1]]);
        assert_eq!(ret, true);
    }

    #[test]
    fn test_2() {
        let ret =
            Solution::find_rotation(vec![vec![0, 1], vec![1, 1]], vec![vec![1, 0], vec![0, 1]]);
        assert_eq!(ret, false);
    }

    #[test]
    fn test_3() {
        let ret = Solution::find_rotation(
            vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]],
            vec![vec![1, 1, 1], vec![0, 1, 0], vec![0, 0, 0]],
        );
        assert_eq!(ret, true);
    }

    #[test]
    fn test_81() {
        let ret =
            Solution::find_rotation(vec![vec![1, 1], vec![0, 1]], vec![vec![1, 1], vec![1, 0]]);
        assert_eq!(ret, true);
    }
}
