use core::num;
use std::collections::HashSet;

struct  Solution;

impl Solution {
    pub fn max_k_distinct(mut nums: Vec<i32>, k: i32) -> Vec<i32> {
        nums.sort();
        let len = nums.len()-1;
        let mut result: Vec<i32> = vec![];
        let mut set: HashSet<i32> = HashSet::new();
        let mut counter = 0;
        let mut iter = 0;
        while iter < k {
            if (len as i32-counter as i32).is_negative() {
                break;
            }
            println!("{:?}", nums[len-counter as usize]);
            if set.contains(&nums[len-counter as usize]) {
                counter +=1;
                continue;
            }
            set.insert(nums[len-counter as usize]);
            result.push( nums[len-counter as usize]);
            counter +=1;
            iter += 1;
        }
        result
    }
}

fn main() {
    println!("{:?}", Solution::max_k_distinct(vec![84,93,100,77,90], 3));
    println!("{:?}", Solution::max_k_distinct(vec![84,93,100,77,93], 3));
    println!("{:?}", Solution::max_k_distinct(vec![1,1,1,2,2,2], 3));

}
