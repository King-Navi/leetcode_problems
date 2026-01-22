struct Solution;

fn main() {
    println!("{:?}", Solution::reverse_prefix("abcd".to_string(), 2));
}

impl Solution {
    pub fn reverse_prefix(s: String, k: i32) -> String {
        let mut c_vec : Vec<char> = s.chars().rev().collect(); 
        let mut reverser_part = String::new();
        for _ in 0..k {
            reverser_part.insert(0,c_vec.pop().unwrap());
        }
        let mut normal_string = String::new();
        for c in c_vec {
            normal_string.insert(0, c);
        }
        
        
        reverser_part.push_str(&normal_string);
        reverser_part
    }
}