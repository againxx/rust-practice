use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut row_existing_nums = vec![HashSet::new(); 9];
        let mut col_existing_nums = vec![HashSet::new(); 9];
        let mut grid_existing_nums = vec![HashSet::new(); 9];
        for i in 0..9 {
            for j in 0..9 {
                let value = board[i][j];
                if value != '.' {
                    let grid_index = i / 3 * 3 + j / 3;
                    if !row_existing_nums[i].insert(value)
                        || !col_existing_nums[j].insert(value)
                        || !grid_existing_nums[grid_index].insert(value)
                    {
                        return false;
                    }
                }
            }
        }
        true
    }
}

fn main() {
    let board = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    println!("{}", Solution::is_valid_sudoku(board));
}
