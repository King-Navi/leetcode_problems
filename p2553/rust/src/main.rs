struct  Solution;
impl Solution {
    pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![];
        for n in nums {
            let s = n.to_string().chars().map(|n| n.to_digit(10).unwrap() as i32).collect::<Vec<i32>>();
            for n_s in s {
                result.push(n_s);
            }
        }
        result
    }
}

fn main() {
    println!("{:?}", Solution::separate_digits(vec![12,13,14]));
    println!("{:?}", Solution::separate_digits(vec![1223134]));
}
