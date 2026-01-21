use std::char;

struct Solution;

fn main() {
    println!("{:?}", Solution::vowel_consonant_score("axeyizou".to_string()));
}
impl Solution {
    pub fn vowel_consonant_score(s: String) -> i32 {
        let chars = s.chars().collect::<Vec<char>>();
        let mut vowels = 0i32;
        let mut consonant = 0i32;
        for c in chars {
            if !c.is_alphabetic() {
                continue;
            }
            if is_vowel(c) {
                vowels +=1;
            }else {
                consonant +=1;
            }
        }
        println!("{:?}",vowels);
        println!("{:?}",consonant);
        if consonant > 0 {
            return vowels/consonant;
        }
        return 0;
    }
}

fn is_vowel(char_actual:char)-> bool{
    match char_actual {
        'a' => true,
        'e' => true,
        'i' => true,
        'o' => true,
        'u' => true,
        _ => false 
    }

}
