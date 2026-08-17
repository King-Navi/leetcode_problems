struct Solution;
impl Solution {
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i32 {
        let mut counter = 0;
        for (i,ni) in nums1.iter().enumerate() {
            for (j,nj) in nums2.iter().enumerate() {
                if nums1[i] % (nums2[j] * k)== 0 {
                    counter +=1;
                }
            }
        }
        counter
    }
}

fn main() {
    println!("Hello, world!");
}
