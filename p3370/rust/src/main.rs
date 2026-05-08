struct Solution;
impl Solution {
    pub fn smallest_number(n: i32) -> i32 {
        let bits = format!("{n:b}").chars().collect::<Vec<char>>();
        let mut string = String::new();
        for c in &bits {
            string.push('1');
        }
        i32::from_str_radix(&string, 2).unwrap()
    }
}
fn main() {
    println!("{}", Solution::smallest_number(5));
}


//  (1 << 32 - n.leading_zeros()) - 1
