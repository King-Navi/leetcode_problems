struct Solution;
impl Solution {
    pub fn is_array_special(nums: Vec<i32>) -> bool {
        if nums.len() <= 1 {
            return true;
        }
        for i in nums.windows(2) {
            if i[0] % 2 == 0 && i[1] % 2 == 0 {
                return false;
            }else if i[0] % 2 == 1 && i[1] % 2 == 1 {
                return false;
            }
        }
        true
    }
}
fn main() {
    println!("Hello, world!");
}
