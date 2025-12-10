struct Solution;

fn main() {
    println!("Hello, world!");
}

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        

        let s_trim =s.trim().chars().collect::<Vec<char>>();
        let mut s = String::new();
        for i in 0 .. s_trim.len() {
            if s_trim[i].is_alphanumeric() {
                s.push(s_trim[i]);    
            }
        }
        let s = s.trim().chars().collect::<Vec<char>>();
        for i in 0 .. s.len()/2  {         
            if s[i].to_ascii_lowercase() != s[s.len() - 1 - i].to_ascii_lowercase() {
                return false;
            }
        }
        true
    }
}
