struct  Solution;
fn main() {
    println!("Hello, world!");
}
impl Solution {
    pub fn transform_array(nums: Vec<i32>) -> Vec<i32> {
        let mut ceros = vec![];
        let mut ones = vec![];
        for n in nums {
            //pair
            if n % 2 == 0{
                ceros.push(0);
                continue;
            } else {
                //odd
                ones.push(1);
                continue;
            }
        }
        [ceros, ones].concat()
    }
}