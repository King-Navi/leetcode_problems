struct Solution;
struct SolutionS;
impl Solution {
    pub fn stable_mountains(height: Vec<i32>, threshold: i32) -> Vec<i32> {        
        let mut i :usize = 1;
        let mut result = Vec::with_capacity(height.len());
        while i < height.len() {
            if height[i-1] > threshold {
                result.push(i as i32);
            }
            i +=1;
        }
        result
    }
}

impl SolutionS {
    pub fn stable_mountains(height: Vec<i32>, threshold: i32) -> Vec<i32> {        
        (1..height.len())
        .filter_map(|i|{
            if height[i-1] > threshold {
                Some(i as i32)
            }else {
                None
            }
        })
        .collect()
    }
}
fn main() {
    println!("{:?}", Solution::stable_mountains(vec![1,2,3,4,5], 2));
}
