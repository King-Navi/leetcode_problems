struct Solution;

impl Solution {
    pub fn minimum_chairs(s: String) -> i32 {
        let mut max_chairs = 0;
        let mut available_chairs = 0;
        for c in s.chars() {
            if c == 'E' {
                if available_chairs == 0 {
                    max_chairs +=1;    
                }else {
                    available_chairs -=1;
                }
                
            }else {
                available_chairs +=1;
            }
        }
        max_chairs
    }
}
fn main() {
    println!("Hello, world!");
}
