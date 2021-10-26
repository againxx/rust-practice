struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let mut i = 0;
        let length = matrix.len();
        while i < length / 2 {
            for k in i..length - i - 1 {
                let mut old_row = i;
                let mut old_col = k;
                let mut new_row = old_col;
                let mut new_col = length - 1 - old_row;
                let mut temp = matrix[old_row][old_col];
                while new_row != i || new_col != k {
                    std::mem::swap(&mut matrix[new_row][new_col], &mut temp);
                    old_row = new_row;
                    old_col = new_col;
                    new_row = old_col;
                    new_col = length - 1 - old_row;
                }
                matrix[i][k] = temp;
            }
            i += 1;
        }
    }
}

fn main() {
    let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    Solution::rotate(&mut matrix);
    println!("{:?}", matrix);
}
