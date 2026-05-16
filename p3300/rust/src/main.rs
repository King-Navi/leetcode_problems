struct Solution;
impl Solution {
    pub fn min_element(nums: Vec<i32>) -> i32 {
        let mut minor = -1;
        for n in nums {
            let num = get_sum(n);
            if minor == -1 {
                minor = num;
            }
            if num<minor {
                minor = num;
            }
        }
        minor
    }
}

#[inline]
pub fn get_sum(n: i32)-> i32{
    let vec = n.to_string().chars().map(|c| c.to_digit(10).unwrap() as i32).collect::<Vec<i32>>();
    let mut sum: i32 =0;
    for num in vec {
        sum +=num;
    }
    sum
}
fn main() {
    println!("Hello, world!");
}



/*
You are given an integer array nums.

You replace each element in nums with the sum of its digits.

Return the minimum element in nums after all replacements.

 

Example 1:

Input: nums = [10,12,13,14]

Output: 1

Explanation:

nums becomes [1, 3, 4, 5] after all replacements, with minimum element 1.

Example 2:

Input: nums = [1,2,3,4]

Output: 1

Explanation:

nums becomes [1, 2, 3, 4] after all replacements, with minimum element 1.

Example 3:

Input: nums = [999,19,199]

Output: 10

Explanation:

nums becomes [27, 10, 19] after all replacements, with minimum element 10.

*/