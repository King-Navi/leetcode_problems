

struct Solution;
fn main() {
    //println!("{}", Solution::count_monobit(1));
    println!("{}", Solution::count_monobit(4));
    //println!("{}", Solution::count_monobit(5));
}

impl Solution {
    pub fn count_monobit(n: i32) -> i32 {
        let mut counter =0;
        for iteration in 0..=n  {
            let mut is_valid = true;
            let binary = String::from(format!("{:b}", iteration));
            let chars = binary.chars().collect::<Vec<char>>();
            let first_char = &chars[0];
            for c in &chars {
                if c != first_char {
                    is_valid = false;
                }

            }

            if is_valid {
                counter +=1;
            }
        }
        counter
    }
}