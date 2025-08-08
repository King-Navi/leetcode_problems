struct  Solution;
fn main() {
    Solution::max_containers(2, 3, 15);
    Solution::max_containers(3, 5, 20);
    Solution::max_containers(2, 1, 3);
}
impl Solution {
    pub fn max_containers(n: i32, w: i32, max_weight: i32) -> i32 {
        let cells = n*n +1;
        let mut output =0;
        for cell in 1..cells {
            if w*cell > max_weight {
                break;
            }
            output +=1;
        }
        println!("{:?}", output);
        output
    }
}