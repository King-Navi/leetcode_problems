use std::cmp;

struct Solution;
impl Solution {
    pub fn minimum_average(nums: Vec<i32>) -> f64 {
        let mut nums = nums;
        let mut result: Vec<f64> = vec![];
        while nums.iter().count() >= 2 {
            let mut max = f64::MIN;
            let mut max_pos = 0_usize;
            let mut min = f64::MAX;
            let mut min_pos = 0_usize;
            {
                for (i, n) in nums.iter().enumerate() {
                    if (*n as f64) < min {
                        min = (*n as f64);
                        min_pos = i;
                    }
                }
                nums.remove(min_pos);
            }
            {
                for (i, n) in nums.iter().enumerate() {
                    if (*n as f64) > max {
                        max = (*n as f64);
                        max_pos = i;
                    }
                }
                nums.remove(max_pos);
            }
            result.push((max + min) / 2.0);
        }

        *result
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
    }
}
fn main() {
    println!(
        "{}",
        Solution::minimum_average(vec![7, 8, 3, 4, 15, 13, 4, 1])
    );
    println!(
        "{}",
        Solution::minimum_average(vec![1,9,8,3,10,5])
    );
    println!(
        "{}",
        Solution::minimum_average(vec![1,2,3,7,8,9])
    );
}
