
pub struct  Solution;


fn main() {
    let s = Solution::longest_nice_substring("dDzeE".to_string());
    println!("{s}");
}
use std::collections::HashSet;

impl Solution {
    pub fn longest_nice_substring(s: String) -> String {
        if s.chars().count() <= 1 {
            return String::new();
        }

        let chars: Vec<char> = s.chars().collect();
        let mut letters = HashSet::new();

        for &c in &chars {
            letters.insert(c);
        }

        let mut new_s : Vec<String> = vec![]; 
        for &c in letters.iter() {
            let counterpart = if c.is_lowercase() {
                c.to_ascii_uppercase()
            } else {
                c.to_ascii_lowercase()
            };
            if !letters.contains(&counterpart) {
                // No es nice, tiene una letra sin pareja
                new_s = s.split(c).map(|s| s.to_string()).collect();
                break;
            }
        }
        if new_s.len() > 1 {
            let mut nices_array_ :Vec<String> = vec![];
            for ns in new_s {
                let s =Solution::longest_nice_substring(ns);
                if !s.is_empty() {
                    nices_array_.push(s);
                }
            }
            if let Some((_, s)) = nices_array_
                .iter()
                .enumerate()
                .max_by_key(|(idx, s)| (s.len(), std::cmp::Reverse(*idx))) 
            {
                return s.clone();
            }
            String::new()
        }else {
            s
        }

        
    }
}