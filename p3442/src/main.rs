
struct  Solution;
fn main() {
    let s  =Solution::max_difference("mmsmsym".to_string());
    println!("{s}");
}
use std::collections::HashMap;

impl Solution {
    pub fn max_difference(s: String) -> i32 {
        let mut hashmap:HashMap<char, i32> = HashMap::new();
        for c in s.chars() {
           *hashmap.entry(c).or_insert(0) +=1; 
        }
        let odd_frec =hashmap.iter().filter(|&(_, x)| x % 2 != 0).max_by_key(|&(_, c)| c).unwrap();
        let even_frec =hashmap.iter().filter(|&(_, x)| x % 2 == 0).min_by_key(|&(_, c)|c).unwrap();
        let result =odd_frec.1 - even_frec.1;
        result
    }
}

