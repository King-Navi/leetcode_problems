
struct Solution;
impl Solution {
    pub fn maximize_expression_of_three(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        nums[nums.len()-1] + nums[nums.len()-2] - nums[0]
    }
}
fn main() {
    println!("{:?}", Solution::maximize_expression_of_three(vec![3,4,1]));
}
