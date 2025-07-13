pub struct Solution;
fn main() {
    Solution::generate_tag(" fPysaRtLQLiMKVvRhMkkDLNedQKffPnCjbITBTOVhoVjiKbfSawvpisDaNzXJctQkn".to_string());
}

impl Solution {
    pub fn generate_tag(caption: String) -> String {
        let mut result = String::new();
        let mut is_next_uppercase : bool = false;
        let mut is_first_uppercase : bool = true;
        result.push('#');
        for (i , c) in caption.chars().into_iter().enumerate() {
            if c == ' ' {
                is_next_uppercase = true;
                continue;
            }
            if is_first_uppercase {
                
                result.push(c.to_ascii_lowercase());
                is_first_uppercase = false;
                is_next_uppercase = false;
                continue;
            }
            
            if is_next_uppercase  {
                is_next_uppercase = false;
                result.push(c.to_ascii_uppercase());
            }else {
                result.push(c.to_ascii_lowercase());
            }
            
            
            if result.chars().count() >= 100 {
                break;
            }
        }
        println!("{:?}", result);
        result
    }
}