struct Solution;
impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut num_cpoy = nums.iter().copied().rev().collect::<Vec<i32>>();
        num_cpoy.sort();
        println!("{:?}", num_cpoy);
        let mut first = true;
        let mut last_number = 0;
        let mut result = 0;
        for n in &num_cpoy {
            if *n < k {
                return -1;
            }
            if first {
                first = false;
                last_number = *n;
            }else if last_number == *n {
                continue;
            } else {
                last_number = *n;
                result +=1;
            }
        }
        if num_cpoy[0] == k {
            result    
        }
        else {
            result + 1
        }
        
    }
}
fn main() {
    println!("{}", Solution::min_operations(vec![10, 8, 10, 8], 2));
}
