
struct Solution;
fn main() {
    Solution::max_sum(vec![1,2,-1,-2,1,0,-1]);
}
use std::collections::HashSet;

impl Solution {
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        let mut hash_set: HashSet<i32> = HashSet::new();
        let mut sum = 0;
        let mut sum_negative = i32::MIN;
        for n in nums {
            if n <= 0 {
                if sum_negative < n {
                    sum_negative = n;
                }
                continue;
            }
            if !hash_set.contains(&n) {
                hash_set.insert(n);
                sum += n;
            }
        }
        println!("{:?}", sum);
        if sum == 0 {
            return sum_negative;
        }
        sum
    }
}