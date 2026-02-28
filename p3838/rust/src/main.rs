struct Solution;
impl Solution {
    pub fn map_word_weights(words: Vec<String>, weights: Vec<i32>) -> String {
        const WORD_A : i32 = 97;
        const WORD_Z : i32 = 122;
        let mut string = String::new();
        for w in words {
            let mut sum = 0;
            for c in w.chars() {
                let index = c as i32 - WORD_A;
                sum += weights[index as usize];
            }
            let result = WORD_Z - sum % 26 ;
            let char = char::from_u32(result as u32).unwrap();
            string.push(char);
        }
        
        string
    }
}
fn main() {
    let a = 'a' as i32;
    let z = 'z' as i32;
    println!("LIMIT A{:?}", a);
    println!("LIMIT Z{:?}", z);
    println!("{:?}", Solution::map_word_weights(
        vec!["abcd".to_string(),"def".to_string(),"xyz".to_string()],
        vec![5,3,12,14,1,2,3,2,10,6,6,9,7,8,7,10,8,9,6,9,9,8,3,7,7,2]));
}
