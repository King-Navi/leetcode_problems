use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut hashmap : HashMap<i32 , i32> = HashMap::new();
        for n in nums {
            *hashmap.entry(n).or_insert(0) += 1;
        }
        if hashmap.len() == 1 {
            return 0;
        }
        1
    }
}

fn main() {
    println!("{}", Solution::min_operations(vec![1,2,5,5]));
}
