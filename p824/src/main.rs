
pub struct Solution;
fn main() {
    let s = Solution::to_goat_latin("I speak Goat Latin".to_string());
    println!("{:?}", s);
}
impl Solution {
    pub fn to_goat_latin(sentence: String) -> String {
        let mut words: Vec<String> = sentence
            .split(" ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let mut new_sentences = String::new();
        let words_len = words.len();
        for (i, w) in words.iter_mut().enumerate() {
            let word_number = i + 1;
            let length_word = w.chars().count() - 1;
            let mut chars: Vec<char> = w.chars().collect();

            if chars[0] == 'a'
                || chars[0] == 'e'
                || chars[0] == 'i'
                || chars[0] == 'o'
                || chars[0] == 'u'
                || chars[0] == 'A'
                || chars[0] == 'E'
                || chars[0] == 'I'
                || chars[0] == 'O'
                || chars[0] == 'U'
            {
                //Only if vowel
                w.push_str("ma"); 
                
            }else {
                //Only if consonant

                if length_word + 1 > 1 {
                    //change to the last place
                    let aux = chars[0];
                    chars.remove(0);
                    chars.push(aux);
                    *w = chars.into_iter().collect();
                    w.push_str("ma");
                }else {
                    w.push_str("ma");
                }
            }
            add_letter_a(word_number, w);
            new_sentences.push_str(w);
            if i != words_len -1 {
                new_sentences.push_str(" ");
            }            
        }
        new_sentences
    }
}

pub fn add_letter_a(number: usize, word: &mut String) {
    for _ in 0..number {
        word.push_str("a");
    }
}
