struct Solution;
impl Solution {
    pub fn earliest_time(tasks: Vec<Vec<i32>>) -> i32 {
        let mut earlist_time = i32::MAX;
        for v in &tasks {
            let sum: i32 = v[0] + v[1];
            println!("{:?}",sum);
            if earlist_time > sum {
                earlist_time = sum;
            }
        }
        earlist_time
    }
}
fn main() {
    println!("{:?}", Solution::earliest_time(vec![vec![1,2]]));
}
