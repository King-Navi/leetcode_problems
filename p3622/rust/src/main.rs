struct Solution;
fn main() {
    println!("{}",Solution::check_divisibility(99));
    println!("{}",Solution::check_divisibility(8));
    println!("{}",Solution::check_divisibility(108));
}

impl Solution {
    pub fn check_divisibility(n: i32) -> bool {
        let num_string = n.to_string().chars().collect::<Vec<char>>();
        let mut sum: i32 = 0;
        let mut product: i32 = -1;
        for c in num_string {
            if let Some(d) = c.to_digit(10) {
                sum += d as i32;
                if product == -1 {
                    product = d as i32;
                } else {
                    product *= d as i32;
                }
            }
        }
        println!("{} , {}", sum, product);
        n % (sum+product)  == 0
    }
}
