use std::io::{self, BufRead};

#[derive(Debug)]
struct Constrains {
    row_constrain: Vec<Vec<bool>>,
    col_constrain: Vec<Vec<bool>>,
    grid_constrain: Vec<Vec<bool>>,
}

impl Constrains {
    fn new(sudoku_grid: &mut Vec<Vec<u32>>) -> Self {
        let row_constrain = vec![vec![false; 9]; 9];
        let col_constrain = vec![vec![false; 9]; 9];
        let grid_constrain = vec![vec![false; 9]; 9];
        let mut constrains = Constrains {
            row_constrain,
            col_constrain,
            grid_constrain,
        };
        for i in 0..9 {
            for j in 0..9 {
                if sudoku_grid[i][j] != 0 {
                    constrains.set(i, j, sudoku_grid[i][j]);
                }
            }
        }
        constrains
    }

    fn validate(&self, row: usize, col: usize, candidate: u32) -> bool {
        let grid_index = row / 3 * 3 + col / 3;
        !self.row_constrain[row][(candidate - 1) as usize]
            && !self.col_constrain[col][(candidate - 1) as usize]
            && !self.grid_constrain[grid_index][(candidate - 1) as usize]
    }

    fn set(&mut self, row: usize, col: usize, value: u32) {
        let grid_index = row / 3 * 3 + col / 3;
        let value = value as usize;
        self.row_constrain[row][value - 1] = true;
        self.col_constrain[col][value - 1] = true;
        self.grid_constrain[grid_index][value - 1] = true;
    }

    fn unset(&mut self, row: usize, col: usize, value: u32) {
        let grid_index = row / 3 * 3 + col / 3;
        let value = value as usize;
        self.row_constrain[row][value - 1] = false;
        self.col_constrain[col][value - 1] = false;
        self.grid_constrain[grid_index][value - 1] = false;
    }
}

fn solve(sudoku_grid: &mut Vec<Vec<u32>>, row: usize, col: usize, constrains: &mut Constrains) -> bool {
    if row == 9 {
        return true;
    }
    let next_row: usize;
    let next_col: usize;
    if col == 8 {
        next_row = row + 1;
        next_col = 0;
    } else {
        next_row = row;
        next_col = col + 1;
    }
    if sudoku_grid[row][col] != 0 {
        return solve(sudoku_grid, next_row, next_col, constrains);
    } else {
        for candidate in 1..10 {
            if constrains.validate(row, col, candidate) {
                sudoku_grid[row][col] = candidate;
                constrains.set(row, col, candidate);
                if solve(sudoku_grid, next_row, next_col, constrains) {
                    return true;
                } else {
                    sudoku_grid[row][col] = 0;
                    constrains.unset(row, col, candidate);
                }
            } else {
                continue;
            }
        }
    }
    false
}

fn main() {
    let mut sudoku_grid: Vec<Vec<u32>> = Vec::new();
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    for line in lines {
        let one_row: Vec<u32> = line
            .unwrap()
            .trim()
            .split(' ')
            .map(|x| x.parse().unwrap())
            .collect();
        sudoku_grid.push(one_row);
    }
    let mut constrains = Constrains::new(&mut sudoku_grid);
    if solve(&mut sudoku_grid, 0, 0, &mut constrains) {
        for i in 0..9 {
            for j in 0..8 {
                print!("{} ", sudoku_grid[i][j]);
            }
            println!("{}", sudoku_grid[i][8]);
        }
    }
}
