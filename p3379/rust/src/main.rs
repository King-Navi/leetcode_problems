use std::ops::BitOrAssign;

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
                println!("{}",nums[i]);
                println!("{:?}",result);
            }else if nums[i] < 0 {
                let mut a: i32 = i as i32 + position as i32;
                if a < 0 {
                    a = lenght as i32 + a ;
                }
                a = a.abs();
                println!("res {}",a);
                //let new_pos = (i as i32 + nums[i]).rem_euclid(lenght as i32);
                let new_pos = ((a % lenght) + length) % length;
                println!("mod {}",new_pos);
                result.push(nums[new_pos as usize]); 
                println!("{}",nums[i]);
                println!("{:?}",result);
            }else {
                result.push(nums[i]);
            }
        }
        result
    }

}
fn main() {
    //println!("{:?}", Solution::construct_transformed_array(vec![3,-2,1,1]));
    //println!("{:?}", Solution::construct_transformed_array(vec![-1,4,-1]));
    //println!("{:?}", Solution::construct_transformed_array(vec![-9,0]));
    println!("{:?}", Solution::construct_transformed_array(vec![3,-2,1,-100]));
}
