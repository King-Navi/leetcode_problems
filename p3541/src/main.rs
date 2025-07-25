
pub struct Solution;
fn main() {
    Solution::max_freq_sum("aeiaeia".to_string());
}
use std::collections::HashMap;

impl Solution {
    pub fn max_freq_sum(s: String) -> i32 {
        let mut frecuency_vowel : HashMap<char , i32> = HashMap::new();
        let mut frecuency_consonant : HashMap<char , i32> = HashMap::new();
        for c in s.chars() {
            if is_vowel(&c) {
                //Vowel
                *frecuency_vowel.entry(c).or_insert(0) +=1;
            }else {
                //consonant
                *frecuency_consonant.entry(c).or_insert(0) +=1;

            }
        }
        let mut result_1 = 0;
        let mut result_2 = 0;
        if let Some((_, &c_vowel)) = frecuency_vowel.iter().max_by_key(|entry| entry.1)  {
            result_1 = c_vowel;
        }
        if let Some((_, &c_cons)) = frecuency_consonant.iter().max_by_key(|entry| entry.1)  {
            result_2 = c_cons;
        }
        println!("{result_1 }   {result_2}");
        result_1 + result_2
    }
}

pub fn is_vowel(c : &char)-> bool{
    let c_lower = &c.to_lowercase().next().unwrap();
    match c_lower {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}