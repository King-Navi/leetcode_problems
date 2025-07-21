//need more work
//
//
//
//
pub struct Solution;
fn main() {
    Solution::smallest_index(vec![1,3,2]);
}

impl Solution {
    pub fn smallest_index(nums: Vec<i32>) -> i32 {
        let mut smalle_index: i32 = i32::MAX;
        for (iter , n) in nums.iter().enumerate()  {
            let nums_chars: Vec<char> = n.to_string().chars().collect();            
            let mut counter : i32= 0; 
            for c in nums_chars {
                counter += c.to_string().parse::<i32>().unwrap();
            }
            if counter == iter as i32 
                && counter < smalle_index {
                smalle_index = iter as i32;
            }
        }
        print!("{smalle_index}");
        if smalle_index == i32::MAX {
            return -1;
        }
        smalle_index
    }
}
