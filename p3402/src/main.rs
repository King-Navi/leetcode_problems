struct Solution;
fn main() {
    println!("Hello, world!");
}

impl Solution {
    pub fn minimum_operations(grid: Vec<Vec<i32>>) -> i32 {
        let mut number_operations = 0;
        for j in 0..grid[0].len() {
            let mut previous_numer = -1;
            for i in 0..grid.len() {
                if previous_numer == -1 {
                    previous_numer = grid[i][j];
                    continue;
                }
                let mut operations = 0;
                if previous_numer >= grid[i][j] {
                    operations = previous_numer - grid[i][j] + 1;
                    number_operations += operations;
                }
                previous_numer = grid[i][j] + operations;
            }
        }

        number_operations as i32
    }
}
