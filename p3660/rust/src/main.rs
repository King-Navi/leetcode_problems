use std::cmp;

struct Solution;
impl Solution {
    pub fn max_value(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut ans = vec![0; n];
        let mut stack: Vec<(i32, usize, usize)> = Vec::new();

        for i in 0..n {
            let mut curr_val = nums[i];
            let mut curr_left = i;
            let curr_right = i;

            while let Some(&(top_val, top_left, _)) = stack.last() {
                if top_val > nums[i] {
                    curr_val = cmp::max(curr_val, top_val);
                    curr_left = top_left;
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push((curr_val, curr_left, curr_right));
        }
        for (val, left, right) in stack {
            for j in left..=right {
                ans[j] = val;
            }
        }
        
        ans
    }
}
fn main() {
    println!("{:?}", Solution::max_value(vec![2, 1, 3]));
    println!("SECOND");
    println!("{:?}", Solution::max_value(vec![2, 3, 1]));
}
