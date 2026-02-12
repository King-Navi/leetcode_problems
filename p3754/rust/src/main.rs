struct Solution;
impl Solution {
    pub fn sum_and_multiply(n: i32) -> i64 {
        let stri = n.to_string();
        let mut result = String::new();
        let mut has_cero = false;
        let mut sum = 0;
        for c in stri.chars() {
            if !c.eq_ignore_ascii_case(&'0') {
                result.push(c);
                sum += i64::from_str_radix(&c.to_string(), 10).unwrap();
                has_cero = true;
            }
        }
        if !has_cero {
            return 0;
        }
        let s = i64::from_str_radix(&result, 10);
        s.unwrap()*sum
    }
}

fn main() {
    println!("Hello, world!");
}
