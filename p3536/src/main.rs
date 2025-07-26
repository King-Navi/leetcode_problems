
pub struct Solution;
fn main() {
    number_to_vec(123);
}

impl Solution {
    pub fn max_product(n: i32) -> i32 {
        let mut v = number_to_vec(n);
        let n_1 = v.pop().unwrap();
        let n_2 = v.pop().unwrap();
        n_1* n_2
    }
}

pub fn number_to_vec(n : i32) -> Vec<i32>{
    if n == 0 {
        return vec![0];
    }
    let mut n = n;
    let mut result = vec![];
    while n >  0 {
        let res = n % 10;
        result.push(res);
        n = n / 10;
    }
    result.sort();
    println!("{:?}", result);
    result
}