
struct Solution;
fn main() {
    println!("Hello, world!");
}

impl Solution {
    pub fn reverse_by_type(s: String) -> String {
        let chars = s.chars().collect::<Vec<char>>();
        let mut vec_pos: Vec<char> = Vec::new();
        let mut vec_char: Vec<char> = Vec::new();
        let mut vec_spec: Vec<char> = Vec::new();
        for c in chars {
            if c.is_alphabetic() {
                vec_pos.push('a');
                vec_char.push(c);
            }else {
                vec_pos.push('e');
                vec_spec.push(c);
            }
        }
        let mut new_s = String::new();
        for c in vec_pos {
            match c {
                'a' => {
                    if let Some(c) = vec_char.pop(){
                        new_s.push(c);
                    }
                    
                },
                'e'=> {
                    if let Some(c) = vec_spec.pop(){
                        new_s.push(c);
                    }

                },
                _ => {

                }
            }
        }

        new_s
    }
}