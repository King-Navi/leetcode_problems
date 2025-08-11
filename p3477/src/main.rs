struct Solution;
fn main() {
    Solution::num_of_unplaced_fruits(vec![3,6,1], vec![6,4,7]);
}
impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, mut baskets: Vec<i32>) -> i32 {
        let mut along_fruits = 0;
        for f in fruits {
            let mut placed = false;
            for w in &mut baskets { 
                if *w >= f {
                    *w = 0;
                    placed = true;
                    break;
                }
            }
            if !placed {
                along_fruits += 1;
            }
        }
        println!("{along_fruits}");
        along_fruits
    }
}