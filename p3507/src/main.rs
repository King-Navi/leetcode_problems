use std::i32;

struct Solution;
fn main() {
    println!("Hello, world!");
}

impl Solution {
    pub fn minimum_pair_removal(mut nums: Vec<i32>) -> i32 {
        let mut operations = 0;

        let is_non_decreasing = |arr: &Vec<i32>| {
            arr.windows(2).all(|w| w[0] <= w[1])
        };

        while !is_non_decreasing(&nums) {
            let mut min_sum = i32::MAX;
            let mut idx = 0;

            for i in 0..nums.len() - 1 {
                let sum = nums[i] + nums[i + 1];
                if sum < min_sum {
                    min_sum = sum;
                    idx = i;
                }
            }

            nums[idx] = nums[idx] + nums[idx + 1];
            nums.remove(idx + 1);

            operations += 1;
        }

        operations
    }
}


// pub fn select_min_sum(mut v : Vec<i32>) -> Vec<i32>{
//     if v.len() <= 2 {
//         return vec![v[0] + v[1]];
//     }
//     let mut s: i32 = i32::MAX;
//     let mut min_sum = (0, 0);
//     for (i , &n) in v.iter().enumerate() {
//         if i == 0 {
//             let current_sum = n + v[ i+1];
//             if current_sum < s {
//                 s = current_sum;
//                 min_sum = (i, i+1);
//             }
//         }
//         if i == v.len() - 1 {
//             let current_sum = n + v[ i-1];

//             if current_sum < s {
//                 return ;
//                 s = current_sum;
//                 min_sum = (i, i+1);
//             }
//         }
//     }
//     vec![]
// }