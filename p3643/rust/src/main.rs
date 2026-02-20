struct Solution;

impl Solution {
    pub fn reverse_submatrix(mut grid: Vec<Vec<i32>>, x: i32, y: i32, k: i32) -> Vec<Vec<i32>> {
        let mut top_pointer = x as usize;
        let mut bottom_poitner = x as usize + k as usize - 1;
        while (bottom_poitner > top_pointer) {
            for colunm in y as usize..(y + k) as usize {
                (grid[top_pointer][colunm], grid[bottom_poitner][colunm]) =
                    (grid[bottom_poitner][colunm], grid[top_pointer][colunm]);
            }
            top_pointer += 1;
            bottom_poitner -= 1;
        }
        return grid;
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::reverse_submatrix(
            vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12],
                vec![13, 14, 15, 16]
            ],
            1,
            0,
            3
        )
    );
    println!(
        "{:?}",
        Solution::reverse_submatrix(vec![vec![19]], 0, 0, 1,)
    );
}
