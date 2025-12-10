use std::collections::HashMap;

struct  Solution;
fn main() {
    println!("Hello, world!");
}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut hmap: HashMap<i32, i32> = HashMap::new();
        for i in 0..nums.len()  {
            *hmap.entry(nums[i]).or_insert(0) +=1;
        }
        if let Some((k,v)) = hmap.iter().find(|(_k, _v)| **_v < 2) {
            
            return  *k;
        }

        0
    }
}