use std::collections::HashMap;

struct Solution;
fn main() {
    let s = Solution::find_valid_pair("2523533".to_string());
    println!("{:?}", s);
}

impl Solution {
    pub fn find_valid_pair(s: String) -> String {
        let v : Vec<char> = s.chars().collect();
        let mut hashmap = HashMap::new();
        for c in &v {
            *hashmap.entry(c).or_insert(0) += 1;
        }
        for (i, &c) in v.iter().enumerate() {
            if i == v.len() - 1 {
                break;
            }

            if let Some((&&key, &value )) = hashmap.get_key_value(&c) 
                && value == c.to_digit(10).unwrap()
                && let Some((&&next_key, &next_value )) = hashmap.get_key_value(&v[i +1])
                && next_value == next_key.to_digit(10).unwrap()  {
                if value == next_value {
                    continue;
                }
                let mut result  = String::new();
                result.push_str(&key.to_string());
                result.push_str(&next_key.to_string());
                return result;
            }
        }
        "".to_string()
    }
}