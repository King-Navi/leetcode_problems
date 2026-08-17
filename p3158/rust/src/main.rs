use std::collections::HashSet;

struct Solution;
impl Solution {
    pub fn duplicate_numbers_xor(nums: Vec<i32>) -> i32 {
        let mut s : HashSet<i32> = HashSet::new();
        let mut vec = vec![];
        for n in nums {
            if s.contains(&n) {
                vec.push(n);
            }else {
                s.insert(n);
            }
        }
        if vec.is_empty() {
            return 0;
        }else if vec.len() == 1 {
            return vec[0];
        }else {
            let mut result  = vec[0];
            for i in 1..vec.len() {
                result = result ^ vec[i];
            }   
            return result ;
        }
    }
}
fn main() {
    println!("Hello, world!");
}
