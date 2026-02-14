use std::cmp::Ordering;

struct Solution;

impl Solution {
    pub fn minimum_flips(n: i32) -> i32 {
        let s_b_original = format!("{n:b}").chars().collect::<Vec<char>>();
        let s_b_flip = s_b_original.iter().rev().cloned().collect::<Vec<char>>();
        if s_b_flip.cmp(&s_b_original) == Ordering::Equal {
            return 0;
        }
        let mut sum = 0; 
        for (i,_) in s_b_original.iter().enumerate() {
            if s_b_original[i] != s_b_flip[i] {
                sum +=1;
            }
        }
        sum
    }
}
fn main() {
    println!("{}", Solution::minimum_flips(2));
}
