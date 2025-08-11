use std::collections::HashSet;

struct  Solution;
fn main() {
    Solution::total_numbers(vec![1,3,5]
);;
    let o = 0_i32;
}
impl Solution {
    pub fn total_numbers(digits: Vec<i32>) -> i32 {
        let mut hashset = HashSet::new();
        let mut counter = 0;
        for (i,first_number) in digits.iter().enumerate()  {
            if first_number == &0 {
                continue;
            }
            for (j, second_number) in digits.iter().enumerate() {
                if i == j  {
                    continue;
                }
                for (k , third_number) in digits.iter().enumerate()  {
                    if k == j || k == i {
                        continue;
                    }
                    let mut s = String::new();
                    s.push_str(&first_number.to_string());
                    s.push_str(&second_number.to_string());
                    s.push_str(&third_number.to_string());
                    let posible_number = s.parse::<i32>().unwrap();
                    if posible_number % 2 == 0 
                        && hashset.insert(posible_number) {
                        counter+=1;
                    }
                }
            }
        }
        println!("{:?}", counter);
        counter
    }
}