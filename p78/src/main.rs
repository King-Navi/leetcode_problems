pub struct Solution;
fn main() {

    Solution::subsets(vec![1,2]);
}

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        result.push(vec![]);
        subsets_cal(&mut result, nums, 0);


        println!("{:?}", result);
        result
    }
}

pub fn subsets_cal(result: &mut Vec<Vec<i32>>, nums: Vec<i32>, num_index : usize) {
    if num_index + 1 > nums.len() {
        return;
    }
    let actual_num = nums[num_index];
    let length_result = result.len();
    for i in 0..length_result {
        let mut actual_array = result[i].clone();
        
        actual_array.push(actual_num);
        result.push(actual_array);
    }
    subsets_cal(result, nums, num_index + 1);
}