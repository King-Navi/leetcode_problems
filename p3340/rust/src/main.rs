struct Solution;
impl Solution {
    pub fn is_balanced(num: String) -> bool {
        let vec :Vec<i32>=num.chars().map(|c|c.to_digit(10).unwrap() as i32).collect();
        let mut sum_odd = 0_i32;
        let mut sum_even = 0_i32;
        for (i,&n) in vec.iter().enumerate() {
            if (i !=0) && i% 2 == 0  {
                //odd
                sum_odd += vec[i]
            }else {
                //even
                sum_even += vec[i]
            }
        }
        sum_even == sum_odd
    }
}
fn main() {
    println!("Hello, world!");
}
