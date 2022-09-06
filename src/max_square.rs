//! dp(i,j)=min(dp(i−1,j),dp(i−1,j−1),dp(i,j−1))+1
use std::cmp::min;

pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
    const ROWS: usize = matrix.len();
    const COLS: usize = matrix[0].len();
    let mut dp = vec![vec![0; ROWS]; COLS];
    let mut ans = 0;
    for i in 0..ROWS {
        for j in 0..COLS {
            if matrix[i][j] == '1' {
                if i == 0 || j == 0 {
                    dp[i][j] = 1;
                } else {
                    dp[i][j] = min(min(dp[i - 1][j], dp[i - 1][j - 1]), dp[i][j - 1]) + 1;
                }
                ans = ans.max(dp[i][j]);
            }
        }
    }
    ans * ans
}