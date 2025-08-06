struct  Solution;
fn main() {
    Solution::min_costs(vec![1,2,4,6,7]);
}

impl Solution {
    pub fn min_costs(cost: Vec<i32>) -> Vec<i32> {
        let mut position = i32::MAX;
        let mut result = vec![];
        for c in cost {
            if position <= c {
                result.push(position);
            }else {
                result.push(c);
                position = c;
            }
        }
        print!("{:?}", result);
        result
    }
}