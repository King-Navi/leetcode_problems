struct Solution;
impl Solution {
    pub fn minimum_index(capacity: Vec<i32>, item_size: i32) -> i32 {
        let mut min_cap_pos: i32 = -1;
        let mut flag = true;
        for (i,cap_box) in capacity.iter().enumerate() {
            if cap_box == &item_size {
                return i as i32;
            }    
            if cap_box > &item_size && flag {
                min_cap_pos = i as i32;
                flag = false;
            }else if cap_box > &item_size && &capacity[min_cap_pos as usize] > cap_box {
                min_cap_pos = i as i32;
            }
        }

        min_cap_pos
    }
}
fn main() {
    println!("Hello, world!");
}
