use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn get_least_frequent_digit(n: i32) -> i32 {
        let n_string = n.to_string().chars().collect::<Vec<char>>();
        let mut frequency: HashMap<char, usize> = HashMap::new();
        for c in n_string {
            print!("{c}");
            *frequency.entry(c).or_insert(0) += 1;
        }
        let result = frequency.iter().min_by(|(d1, c1), (d2, c2)| 
            c1.cmp(c2).then_with(|| d1.cmp(d2))
        ).unwrap();
        println!("{:?}", result);
        result.0.to_digit(10).unwrap() as i32
    }
}
fn main() {
    println!("{:?}", Solution::get_least_frequent_digit(1553322));
    println!("{:?}", Solution::get_least_frequent_digit(723344511));
}
