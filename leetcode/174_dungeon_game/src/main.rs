struct Solution;

impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let height = dungeon.len();
        let width = dungeon[0].len();
        let mut minimum_hp: Vec<Vec<i64>> = vec![vec![i32::MAX as i64; width + 1]; height + 1];
        minimum_hp[height][width - 1] = 1;
        minimum_hp[height - 1][width] = 1;
        for i in (0..height).rev() {
            for j in (0..width).rev() {
                let down_minimum = (minimum_hp[i + 1][j] - dungeon[i][j] as i64).max(1);
                let right_minimum = (minimum_hp[i][j + 1] - dungeon[i][j] as i64).max(1);
                minimum_hp[i][j] = (down_minimum).min(right_minimum);
            }
        }
        minimum_hp[0][0] as i32
    }
}

fn main() {
    let input = vec![vec![-2,-3,3], vec![-5,-10,1], vec![10,30,-5]];
    println!("{}", Solution::calculate_minimum_hp(input));
}
