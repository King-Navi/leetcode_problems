struct Solution;

impl Solution {
    pub fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
        let mut return_vec = vec![];
        for n in nums {
            println!("------{n}------");
            let mut number = 1;
            while number<=n {
                let result = number as u32 | (number+1) as u32;   
                if result == n as u32 {
                    return_vec.push(number as i32);
                    break;
                } else {
                    number +=1;
                }
            }
            if number>n {
                return_vec.push(-1);
            }
        }
        return_vec
    }
}

fn main() {
    //println!("{:?}",Solution::min_bitwise_array(vec![2,3,5,7]));
    //println!("{:?}",Solution::min_bitwise_array(vec![11,13,31]));
    println!("{:?}",Solution::min_bitwise_array(vec![521]));
}


/*
You are given an array nums consisting of n prime integers.

You need to construct an array ans of length n, such that, for each index i, the bitwise OR of ans[i] and ans[i] + 1 is equal to nums[i], i.e. ans[i] OR (ans[i] + 1) == nums[i].

Additionally, you must minimize each value of ans[i] in the resulting array.

If it is not possible to find such a value for ans[i] that satisfies the condition, then set ans[i] = -1.


*/