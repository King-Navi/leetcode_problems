struct Solution;

impl Solution {
    pub fn min_changes(n: i32, k: i32) -> i32 {
        if n == k {
            return 0;
        }
        if (n & k) !=k {
            return -1;
        }
        (n ^ k).count_ones() as i32
    }
}
fn main() {
    println!("Hello, world!");
}
