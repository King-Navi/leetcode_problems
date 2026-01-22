struct Solution;

fn main() {
    println!("{}", Solution::largest_even("1112".to_string()));
    println!("{}", Solution::largest_even("221".to_string()));
    println!("{}", Solution::largest_even("".to_string()));
    println!(
        "{}",
        Solution::largest_even("11112111222122121221".to_string())
    );
}


impl Solution {
    pub fn largest_even(mut s: String) -> String {
        if let Some(c) = s.pop(){
            if c == '2' {
                s.push(c);
                return s;
            }else {
                return Self::largest_even(s);    
            }
        }
        String::new()
    }
}
