struct Solution;
impl Solution {
    pub fn recover_order(order: Vec<i32>, friends: Vec<i32>) -> Vec<i32> {
        let mut result : Vec<i32> = vec![]; 
        for o in order {
            for f in &friends {
                if f == &o {
                    result.push(*f);
                }
            }
        }
        result
    }
}
fn main() {
    println!("Hello, world!");
}
