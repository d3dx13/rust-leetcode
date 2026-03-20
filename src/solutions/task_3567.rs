#![allow(unused)]
struct Solution;

impl Solution {
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
