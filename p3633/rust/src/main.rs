use std::{cmp::max, i32};

struct Solution;
fn main() {
    println!("Hello, world!");
}

impl Solution {
    pub fn earliest_finish_time(land_start_time: Vec<i32>, 
        land_duration: Vec<i32>, 
        water_start_time: Vec<i32>, 
        water_duration: Vec<i32>) -> i32 {
        let mut ans = i32::MAX;

        for i in 0..land_start_time.len() {
            for j in 0..water_start_time.len() {
                let finish = max(
                    land_start_time[i] + land_duration[i],
                    water_start_time[j],
                ) + water_duration[j];

                ans = ans.min(finish);
            }
        }

        for i in 0..water_start_time.len() {
            for j in 0..land_start_time.len() {
                let finish = max(
                    water_start_time[i] + water_duration[i],
                    land_start_time[j],
                ) + land_duration[j];

                ans = ans.min(finish);
            }
        }

        ans
    }
}