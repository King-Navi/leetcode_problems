
struct Solution;
impl Solution {
    pub fn is_trionic(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut i = 1;
        while i < n && nums[i] > nums[i - 1] {
            i += 1;
        }    
        if i == 1 || i == n {
            return false;
        }

        while i < n && nums[i] < nums[i - 1] {
            i += 1;
        }    
        if i == 1 || i == n {
            return false;
        }

        while i < n && nums[i] > nums[i - 1] {
            i += 1;
        }    
        i == n  
    }
}


fn main() {
    println!("{:?}", Solution::is_trionic(vec![1,2,3]));
}
