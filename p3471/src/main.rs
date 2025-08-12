struct Solution;
fn main() {
    let s = Solution::largest_integer(vec![3,9,2,1,7], 3);
    println!("{s}");
    let s = Solution::largest_integer(vec![3,9,7,2,1,7], 4);
    println!("{s}");
    let s = Solution::largest_integer(vec![0,0], 1);
    println!("{s}");
    let s = Solution::largest_integer(vec![3,3,3], 3);
    println!("{s}");
    let s = Solution::largest_integer(vec![1,3,7,9,2,7], 2);
    println!("{s}");
    let s = Solution::largest_integer(vec![8,1,8], 3); // 8
    println!("{s}");
    let s = Solution::largest_integer(vec![0,0], 2); // 0
    println!("{s}");
    let s = Solution::largest_integer(vec![11,0,11,0,0,3,3,8], 4);
    println!("{s}");
}
use std::collections::HashMap;

impl Solution {
    pub fn largest_integer(nums: Vec<i32>, k: i32) -> i32 {
        let mut hashmap: HashMap<i32,i32> = HashMap::new();
        for &n in &nums {
            *hashmap.entry(n).or_insert(0) +=1;
        }
        if k == 1 {
            if let Some(s) =hashmap.iter().filter(|(k, v)| hashmap.get(&k).map_or(false, |&v| v == 1)).max(){
                return *s.0;
            }else {
                return -1;
            }
        }
        let last = nums[nums.len() - 1];

        if k as usize == nums.len() {
            return *hashmap.keys().max().unwrap();
        }
        if last > nums[0] && hashmap.get(&last).map_or(false, |&v| v == 1) {
            return last;
        }else if hashmap.get(&nums[0]).map_or(false, |&v| v == 1)  {
            return nums[0];
        }
        else if hashmap.get(&last).map_or(false, |&v| v == 1)  {
            return last;
        }
        -1
    }
}