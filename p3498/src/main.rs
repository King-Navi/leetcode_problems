struct  Solution;
fn main() {
    Solution::reverse_degree("abc".to_string());
}


impl Solution {
    pub fn reverse_degree(s: String) -> i32 {
        let mut sum = 0;
        for (i , c) in s.chars().enumerate() {
            let num = (c as i32 - 123).abs();
            sum += num * (i as i32 + 1) ;

        }
        sum 
    }
}