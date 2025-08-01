pub struct Solution; 
fn main() {
    println!("Hello, world!");
}

//
//
//
impl Solution {
    pub fn count_hill_valley(nums: Vec<i32>) -> i32 {
        let last_index: usize = nums.iter().count(); 
        for (i , n) in nums.iter().enumerate() {
            if i == 0  {
                continue;
            }
            if i == nums.iter().count() - 1 {
                break;
            }
            //hill
            if nums[i-1] < {
                
            }

        }
        0
    }
}

pub fn check_next_hill() -> (i32, i32){

}

pub fn check_next_valley() -> (i32, i32){
    
}