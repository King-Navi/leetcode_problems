struct Solution;
struct Improve;
impl Solution {
    pub fn possible_string_count(word: String) -> i32 {
        let mut counter: i32 = 0
        ; 
        let vec = word.chars().collect::<Vec<char>>();
        for (i, &c) in vec.iter().enumerate() {
            if i +1 < vec.len() && vec[i+1] == c {
                    counter +=1;
                }
           

        }
        counter +1
    }
}

impl Improve {
    pub fn possible_string_count(word: String) -> i32 {
        let bytes = word.as_bytes();
        
        let mut extra_chars = 0;

        for window in bytes.windows(2) {
            if window[0] == window[1] {
                extra_chars += 1;
            }
        }

        extra_chars + 1
    }
}
fn main() {
    let word= "abbcccc".to_string();
    

    let bytes = word.as_bytes().to_vec();
    println!("{bytes:?}");
    let str = String::from_utf8(bytes).unwrap();
    println!("{str:?}");


    println!("{:?}",Solution::possible_string_count(word.clone()));
}
