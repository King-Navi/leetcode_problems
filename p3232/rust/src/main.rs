struct Solution;
impl Solution {
    pub fn can_alice_win(nums: Vec<i32>) -> bool {
        let mut sum_single = 0;
        let mut sum_double = 0;
        for n in nums {
            if n <= 9 {
                sum_single += n;
            }else {
                sum_double += n;
            }
        }
        if sum_double == sum_single {
            return false;
        }
        true // alice win
    }
}
fn main() {
    println!("Hello, world!");
}
