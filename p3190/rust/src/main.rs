pub struct Solution;
impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut counter = 0;
        for n in nums {
            if n % 3 == 0 {
                continue;
            }
            let mut substract = n;
            let mut plus = n;
            loop {
                substract -=1;
                plus +=1;
                counter +=1;
                if substract % 3 == 0 {
                    break;
                }else if plus % 3 == 0    {
                    break;
                }
            }
        }
        counter
    }
}

fn main() {
    println!("Hello, world!");
}
