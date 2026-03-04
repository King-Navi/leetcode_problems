struct Solution;
impl Solution {
    pub fn trim_trailing_vowels(s: String) -> String {
        let mut resutl = String::new();
        let v_char = s.chars().rev().collect::<Vec<char>>();
        let mut is_over = false;
        for (i, c) in v_char.iter().enumerate() {
            
            if is_over {
                resutl.push(*c);
                continue;
            }
            if let Some(next_char) = v_char.get(i + 1) {
                if is_vowel(c) && !is_vowel(next_char) {
                    println!("{:?}",c);
                    println!("{:?}",next_char);
                    is_over = true;
                    continue;
                } else if !is_vowel(c) && is_vowel(next_char) {
                    resutl.push(*c);
                    
                    is_over = true;
                    continue;
                } else if !is_vowel(c) && !is_vowel(next_char) {
                    resutl.push(*c);
                    is_over = true;
                    continue;
                } else if is_vowel(c) && is_vowel(next_char) {
                    continue;
                }
            } else {
                if is_vowel(c) {
                    return resutl;
                } else {
                    resutl.push(*c);
                    return resutl;
                }
            }
        }
        resutl.chars().rev().collect::<String>()
    }
}
fn main() {
    println!("{:?}", Solution::trim_trailing_vowels("ida".to_string()));
}

fn is_vowel(c: &char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}
