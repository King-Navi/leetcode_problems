struct NeighborSum {
    grid: Vec<Vec<i32>>

}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NeighborSum {
    fn new(grid: Vec<Vec<i32>>) -> Self {
        NeighborSum { grid }

    }

    fn adjacent_sum(&self, value: i32) -> i32 {
        let (i,j) = self.get_index(value);
        let mut sum = 0;
        if i as i32 -1 >=0 {
            sum += self.grid[i-1][j]
        }
        if j as i32 -1 >= 0 {
            sum += self.grid[i][j-1]
        }
        if i+1 <= self.grid.len() -1{
            sum += self.grid[i+1][j]
        }

        if j+1 <= self.grid[i].len() -1 {
            sum += self.grid[i][j+1]
        }
        sum
    }

    fn diagonal_sum(&self, value: i32) -> i32 {
        let (i,j) = self.get_index(value);
        let i = i as i32;
        let j = j as i32;
        let mut sum = 0;
        if i-1 >= 0 && j-1 >= 0{
            sum += self.grid[i as usize - 1 ][j as usize -1];
        }
        if i-1 >= 0 && j+1 <= self.grid[i as usize].len() as i32 -1{
            sum += self.grid[i as usize - 1 ][j as usize +1];

        }

        if i+1 <= self.grid.len() as i32 -1 && j-1 >= 0{
            sum += self.grid[i as usize + 1 ][j as usize - 1];
        }
        if i+1 <= self.grid.len() as i32 -1 && j+1 <= self.grid[i as usize].len() as i32 -1{
            sum += self.grid[i as usize + 1 ][j as usize + 1];
            
        }

        sum
    }
    fn get_index(&self, value: i32) -> (usize, usize){
        for (i, row) in self.grid.iter().enumerate() {
            for (j, _) in row.iter().enumerate() {
                if value == self.grid[i][j] {
                    return (i,j);
                }
            }
        }
        (0,0)
    }
}

/**
 * Your NeighborSum object will be instantiated and called as such:
 * let obj = NeighborSum::new(grid);
 * let ret_1: i32 = obj.adjacent_sum(value);
 * let ret_2: i32 = obj.diagonal_sum(value);
 */
fn main() {
    // let obj = NeighborSum::new(vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]]);
    // let ret_1: i32 = obj.adjacent_sum();
    // let ret_2: i32 = obj.diagonal_sum();
}

/*
3242. Design Neighbor Sum Service

You are given a n x n 2D array grid containing distinct elements in the range [0, n^2 - 1].

Implement the NeighborSum class:

NeighborSum(int [][]grid) initializes the object.

int adjacentSum(int value) returns the sum of elements which are adjacent neighbors of value, that is either to the top, left, right, or bottom of value in grid.

int diagonalSum(int value) returns the sum of elements which are diagonal neighbors of value, that is either to the top-left, top-right, bottom-left, or bottom-right of value in grid.




Example 1:

Input:

["NeighborSum", "adjacentSum", "adjacentSum", "diagonalSum", "diagonalSum"]

[[[[0, 1, 2], [3, 4, 5], [6, 7, 8]]], [1], [4], [4], [8]]

Output: [null, 6, 16, 16, 4]

Explanation:



The adjacent neighbors of 1 are 0, 2, and 4.
The adjacent neighbors of 4 are 1, 3, 5, and 7.
The diagonal neighbors of 4 are 0, 2, 6, and 8.
The diagonal neighbor of 8 is 4.
Example 2:

Input:

["NeighborSum", "adjacentSum", "diagonalSum"]

[[[[1, 2, 0, 3], [4, 7, 15, 6], [8, 9, 10, 11], [12, 13, 14, 5]]], [15], [9]]

Output: [null, 23, 45]

Explanation:



The adjacent neighbors of 15 are 0, 10, 7, and 6.
The diagonal neighbors of 9 are 4, 12, 14, and 15.


Constraints:

3 <= n == grid.length == grid[0].length <= 10
0 <= grid[i][j] <= n2 - 1
All grid[i][j] are distinct.
value in adjacentSum and diagonalSum will be in the range [0, n2 - 1].
At most 2 * n2 calls will be made to adjacentSum and diagonalSum.
*/
