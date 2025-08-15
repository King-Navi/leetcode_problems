struct Solution;
fn main() {
    let b =Solution::has_same_digits("3902".to_string());
    println!("{b}");
    let b =Solution::has_same_digits("34789".to_string());
    println!("{b}");
    let b =Solution::has_same_digits("39023".to_string());
    println!("{b}");
}

impl Solution {
    pub fn has_same_digits(s: String) -> bool {
        let result: String = min_s(s);
        let vec: Vec<char> = result.chars().collect();
        if vec[0] == vec[1] {
            return true;
        }
        false
    }
}

pub fn min_s(s: String) -> String {
    if s.chars().count() == 2 {
        return s;
    }
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();

    for (i, &c) in chars.iter().enumerate() {
        if i + 1 < chars.len() {
            let next = chars[i + 1];
            let operation = (c as usize + next as usize) % 10;
            result.push_str(&operation.to_string());
        }
    }
    return min_s(result);
}
