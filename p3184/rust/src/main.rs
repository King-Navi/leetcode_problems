use std::result;

struct Solution;
impl Solution {
    pub fn count_complete_day_pairs(hours: Vec<i32>) -> i32 {
        let mut result = 0;
        for i in 0..hours.len() {
            let mut second_hour_index = i+1;
            loop {
                if second_hour_index > hours.len() -1 {
                    break;
                }else {
                    if (hours[i] + hours[second_hour_index]) % 24 == 0 {
                        result +=1;
                    }
                }
                second_hour_index +=1;
            }
        }
        result
    }
}
fn main() {
    println!("Hello, world!");
}
