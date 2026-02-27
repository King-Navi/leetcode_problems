use std::collections::HashSet;

struct  Solution;
impl Solution {
    pub fn smallest_absent(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut set: HashSet<i32> = HashSet::new();
        for &n in &nums {
            sum += n;
            set.insert(n);
            
        }
        sum /= nums.len() as i32;
        let mut counter = if sum > 0 { sum + 1 } else { 0 + 1 };
        loop {
            if !set.contains(&counter) {
                return counter;
            }    
            counter +=1;
        }
    }
}

fn main() {
    println!("Hello, world!");
}
