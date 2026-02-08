use std::i32;

struct Solution;
impl Solution {
    pub fn abs_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        let size_nums = nums.iter().count();
        if size_nums == 1 {
            return 0;
        }
        let mut largest_sum: i32 = 0;
        let mut smallest_sum: i32 = 0;
        nums.sort();

        let min_num : &[i32] = &nums[..k as usize];
        println!("{:?}", min_num);
        for n in min_num {
            smallest_sum += n;
        }
        let start = size_nums -k as usize;
        let max_num : &[i32] = &nums[start ..];
        for n in max_num {
            largest_sum += n;
        }
        println!("{:?}", max_num);
        (largest_sum -smallest_sum).abs()
    }
}
fn main() {
    println!("{}", Solution::abs_difference(vec![100, 3,2], 1));
}
