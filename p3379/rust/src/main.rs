struct Solution;
impl Solution {
    pub fn construct_transformed_array(nums: Vec<i32>) -> Vec<i32> {
        let lenght = nums.len(); 
        let mut result: Vec<i32> = vec![];
        for i in 0..lenght {
            let position = nums[i];
            if nums[i] > 0  {
                let a: i32 = i as i32 + position;
                let new_pos = a % (lenght as i32);
                result.push(nums[new_pos as usize]);
                println!(" {} new position {}", a , new_pos);
            }else if nums[i] < 0 {
                let a: i32 = i as i32 - position;
                let new_pos = a.abs() % (lenght as i32);
                result.push(nums[new_pos as usize]); 
                println!("{} {}", a , new_pos);
            }else {
                result[i] = nums[i];
            }
        }
        result
    }

}
fn main() {
    println!("{:?}", Solution::construct_transformed_array(vec![3,-2,1,1]));
    println!("{:?}", Solution::construct_transformed_array(vec![-1,4,-1]));
}
