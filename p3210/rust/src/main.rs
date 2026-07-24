struct Solution;

impl Solution {
    pub fn get_encrypted_string(s: String, k: i32) -> String {
        let mut result = String::new();
        let mut vec_char = s.chars().collect::<Vec<char>>();
        for i in 0..s.len() {
            result.push_str(&vec_char[(i+k as usize)%vec_char.iter().count()].to_string());
        }
        result
    }
}
fn main() {
    println!("Hello, world!");
}
