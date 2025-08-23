struct Solution;
fn main() {
    Solution::subarray_sum(vec![3,2,1]);
}

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        for (i , _) in nums.iter().enumerate() {
            let mut start = i  as i32 - nums[i];
            if start < 0 {
                start = 0;
            }
            let stop = i + 1; 
            let slice = &nums[start as usize.. stop];
            for s in slice  {
                result += s;
            }
        }
        result      
    }
}