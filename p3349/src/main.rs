struct  Solution;


impl Solution {
    pub fn has_increasing_subarrays(nums: Vec<i32>, k: i32) -> bool {
        if k == 1 {
            return true;
        }
        let mut count =1;
        let mut vec =vec![];
        for i in 0..nums.len()-1 {    
            if nums[i] < nums[i+1] {
                count +=1;
            }else {
                count = 1;
            }
            if count >= k {
                vec.push(i+1);
            }
        }
        if vec.is_empty() {
            return false;
        }
        println!("{:?}",vec);
        for i in 0..vec.len() {
            for j in i+1..vec.len()  {
                if vec[i]+k as usize==vec[j] {
                    return true;
                }
            }
        }
        false
     }
}



fn main() {
    println!("{}",Solution::has_increasing_subarrays(vec![2,5,7,8,9,2,3,4,3,1], 3));//true
    println!("2nd case {}",Solution::has_increasing_subarrays(vec![1,2,6,6,6,4,5], 2)); //false
    println!("3rd case{}",Solution::has_increasing_subarrays(vec![-15,19], 1));//true
    println!("4th case {}",Solution::has_increasing_subarrays(vec![-9,9,12,3,6], 2));//true
    println!("5th case {}",Solution::has_increasing_subarrays(vec![1,2,3,4,4,4,4,5,6,7], 5));//false
    println!("6th case {} expected true",Solution::has_increasing_subarrays(vec![1,2,3,4,5], 2));//true
    println!("7th case {} expected false",Solution::has_increasing_subarrays(vec![-15,3,16,0], 2));//false
    println!("8th case {} expected true",Solution::has_increasing_subarrays(vec![-9,-8,-5,7,9,19], 3));//true
    println!("9th case {} expected true",Solution::has_increasing_subarrays(vec![5,8,-2,-1], 2));//true
}
