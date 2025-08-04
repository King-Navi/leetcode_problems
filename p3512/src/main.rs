pub struct Solution;
fn main() {
    Solution::min_operations(vec![4,1,3], 5);
}

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut sum = 0;
        for n in nums {
            sum += n;
        }
        if sum % k == 0 {
            println!("0");
            return 0;
        }
        let mut decreses = 0;
        while sum % k != 0 {
            sum -=1;
            decreses +=1;
            if sum <= 0 {
                break;
            }
        }
        println!("{:?}", decreses);
        decreses
    }
}
