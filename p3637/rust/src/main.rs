
struct Solution;
impl Solution {
    pub fn is_trionic(nums: Vec<i32>) -> bool {
        let mut last_number: &i32 = &nums[0];
        let max_index =nums.len() -1;
        let mut last_index =nums.len() -1;
        for i in 1..=max_index {
            if last_number < &nums[i] {
                last_number = &nums[i];
            }else {
                last_number = &nums[i];
                last_index = i;
                break;
            }
            return false;
        }

        for i in last_index..=max_index {
            if last_number > &nums[i] {
                last_number = &nums[i];
            }else {
                last_index = i;
                break;
            }
            return false;
        }
        false
    }
}


fn main() {
    println!("{:?}", Solution::is_trionic(vec![1,2,3]));
}
