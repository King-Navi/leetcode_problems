use std::ops::Index;


struct  Solution;
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() == 1 {
            return s;
        }
        let vec_char: Vec<char> = s.chars().collect();
        let mut start_result = 0;
        let mut end_result = 0;
        for index in 0..s.len() {
            let mut left: i32 = index as i32;
            let mut right  = index;
            while 
                left >= 0 
                && right < vec_char.len() 
                && vec_char[left as usize] == vec_char[right] {
                    if vec_char[left as usize..=right].len() > end_result-start_result {
                        end_result = right;
                        start_result = left as usize;
                    }
                right +=1;
                left -=1;
            }

            let mut left: i32 = index as i32;
            let mut right  = index +1;
            while 
                left >= 0 
                && right < vec_char.len() 
                && vec_char[left as usize] == vec_char[right] {
                    if vec_char[left as usize..=right].len() > end_result-start_result {
                        end_result = right;
                        start_result = left as usize;
                    }
                right +=1;
                left -=1;
            }
            
        }
        let s: String = vec_char[start_result as usize..=end_result].into_iter().collect();
        s
    }

}


fn main() {
    // println!("{:?}",is_palindrome(&['a','a','a']));
    // println!("{:?}",is_palindrome(&['a','a','b']));
    // println!("{:?}",is_palindrome(&['a','a']));
    // println!("{:?}",is_palindrome(&['a','b']));
    
    println!("{}", Solution::longest_palindrome("babad".to_string()));
    println!("{}", Solution::longest_palindrome("ccc".to_string()));
    println!("{}", Solution::longest_palindrome("cbbd".to_string()));
    println!("{}", Solution::longest_palindrome("ac".to_string()));
    println!("{}", Solution::longest_palindrome("a".to_string()));
}
