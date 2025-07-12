
pub struct Solution;

fn main() {
    Solution::check_prime_frequency(vec![9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9]);
}
use std::collections::HashMap;

impl Solution {
    pub fn check_prime_frequency(nums: Vec<i32>) -> bool {
        let mut hashmap : HashMap<i32, i32>= HashMap::new();

        for &num in nums.iter() {
            *hashmap.entry(num).or_insert(0) += 1;
        }
        for &i in hashmap.values() {
            if i == 1 {
                continue;
            }
            if i != 2 && i % 2 ==0 {
                continue;
            }
            if i != 3 && i % 3 ==0 {
                continue;
            }
            if i != 5 && i % 5 ==0 {
                continue;
            }
            if i != 7 && i % 7 ==0 {
                continue;
            }
            
            if i % i == 0 && i % 1 == 0 {
                return true;
            }    
        }
        
        print!("{:?}", hashmap);


        false
    }
}