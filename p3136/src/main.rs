pub struct  Solution;
fn main() {
    let s = Solution::is_valid("AhI".to_string());
    println!("{s}");
}

impl Solution {
    pub fn is_valid(word: String) -> bool {
        //Return true if word is valid, otherwise, return false.
        let letters : Vec<char> = word.chars().collect();
        let (mut one_vowel , mut one_consonant) = (false, false);
        if &letters.len() < &3{
            return false;
        }
        for c in letters  {
            if c.is_numeric() {
                continue;
            }
            if c.is_alphabetic() && is_vowel(&c) {
                one_vowel = true;
                continue;
            }else if c.is_alphabetic() {
                one_consonant = true;
                continue;
            }
            if !c.is_alphanumeric() {
                return false;
            }

        }
        one_vowel && one_consonant
    }
}

pub fn is_vowel(c : &char) -> bool{
    let c_lower = c.to_lowercase().next().unwrap();
    match c_lower {
       'a' => return true,
       'e' => return true,
       'i' => return true,
       'o' => return true,
       'u' => return true,
       _ =>  return false
    }
}