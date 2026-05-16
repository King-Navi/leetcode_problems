struct Solution;

impl Solution {
    pub fn smallest_number(n: i32, t: i32) -> i32 {
        let mut n = n;
        while check_condition(n) % t != 0 {
            n +=1;
        }
        n

    }
}
#[inline]
pub fn check_condition(n:i32)-> i32{
    let digits = n
        .to_string()
        .chars()
        .map(|c | c.to_digit(10).unwrap() as i32)
        .collect::<Vec<i32>>();
    let mut coutner = 1;
    for d in digits {
        coutner = d * coutner;
    }
    coutner
}
fn main() {
    println!("{}",Solution::smallest_number(10, 2));
    println!("{}",Solution::smallest_number(15, 3));
}
