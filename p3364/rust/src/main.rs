struct Solution;
impl Solution {
    pub fn minimum_sum_subarray(nums: Vec<i32>, l: i32, r: i32) -> i32 {
        let mut result = -1;
for size in l..=r {
    nums.windows(size as usize).enumerate().for_each(|(i, window)| {
            let mut sum = 0; 
            
            for n in window {
                sum += n;
            }
            if sum > 0 
                && (sum < result || result == -1) {
                result = sum;
            }
            println!("Window {i} {window:?}");
        });
}
        result
    }
}
fn main() {
    println!("{}", Solution::minimum_sum_subarray(vec![3, -2, 1, 4], 2, 3));
    println!("{}", Solution::minimum_sum_subarray(vec![-2, 2, -3, 1], 2, 3));
    println!("{}", Solution::minimum_sum_subarray(vec![1,2,3,4], 2, 4));
    println!("{}", Solution::minimum_sum_subarray(vec![5,8,-6], 1, 3));
}
