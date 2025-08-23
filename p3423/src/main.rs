struct Solution;
fn main() {
    let s = Solution::max_adjacent_distance(vec![1,2,4]);
    println!("{:?}", s);
}
impl Solution {
    pub fn max_adjacent_distance(nums: Vec<i32>) -> i32 {
        let stop_index = nums.len() - 1;
        let mut result: i32 = nums[0] - nums[stop_index];
        result = result.abs();
        for (i, n) in nums.iter().enumerate() {
            if i == stop_index{
                break;
            }
            let temp = (nums[i] - nums[i+1]).abs();
            if temp > result {
                result = temp;
            }
        }
        result
    }
}