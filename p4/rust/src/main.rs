struct Solution;
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2:  Vec<i32>) -> f64 {
        let mut vec : Vec<f64> = vec![];
        {
            let mut len_1 = 0;
            let mut len_2 = 0;
            while len_1 < nums1.len() && len_2 < nums2.len() {
                if nums1[len_1] < nums2[len_2] {
                    vec.push(nums1[len_1] as f64);
                    len_1 +=1;
                }else {
                    vec.push(nums2[len_2] as f64);
                    len_2 +=1;
                }
                
            }
            while len_1 < nums1.len() {
                    vec.push(nums1[len_1] as f64);
                    len_1 +=1;
                
            }
            while len_2 < nums2.len() {
                    vec.push(nums2[len_2] as f64);
                    len_2 +=1;
                
            }
        }
        
        let position: usize = vec.len()/2;
        if vec.len() == 1 {
            return vec[0];
        }
        if vec.len() % 2 == 0 {
            //even
            return (vec[position] + vec[position-1])/2_f64;
        }else {
            //odd
            return vec[position];
        }
    }
}
fn main() {
    println!("{:?}", Solution::find_median_sorted_arrays(vec![0,0,0,0,0]
, vec![-1,0,0,0,0,0,1]));
}
