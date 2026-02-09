struct Solution;
impl Solution {
    pub fn dominant_indices(nums: Vec<i32>) -> i32 {
        let mut dominant = 0;
        let max_index = nums.iter().count() - 1;
        for (i , n) in nums.iter().enumerate() {
            if i == max_index {
                break;
            }
            if n > &check_average(&nums[i..]) {
                dominant +=1;
            }
        }
        dominant
    }
}

fn check_average(slice_i32 : &[i32])-> i32{
    let mut sum=0;
    let element = slice_i32.iter().count();
    for n in slice_i32 {
        sum +=n
    }
    sum/element as i32
}

fn main() {
    println!("{}", Solution::dominant_indices(vec![5,4,3]));
}
