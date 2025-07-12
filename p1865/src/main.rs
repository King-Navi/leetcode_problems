
fn main() {
    let mut obj = FindSumPairs::new(
        vec![9,70,14,9,76],
        vec![26,26,58,23,74,68,68,78,58,26]
    );
    // obj.count(7);
    // obj.add(3, 2);
    // obj.count(8);
    // obj.count(4);
    // obj.add(0, 1);
    // obj.add(1,1);
    // obj.count(7);
    obj.add(6, 10);
    obj.add(5, 6);
    obj.count(32);

}


use std::{collections::HashMap};

struct FindSumPairs {
    //num1 doesn't change
    nums1: Vec<i32>,
    //num2 change, we care about the position, the length is always the same
    nums2: Vec<i32>,
    //Frecuency
    freq_map: HashMap<i32, i32>,
}

impl FindSumPairs {

    fn new(nums1: Vec<i32>, nums2: Vec<i32>) -> Self {
        let mut copy_1 = nums1.clone();
        copy_1.sort();
        Self { nums1: copy_1, nums2: nums2.clone(), freq_map: update_map(&nums2) }
    }
    
    fn add(&mut self, index: i32, val: i32) {
        let idx = index as usize;
        let old_val = self.nums2[idx];

        if let Some(freq) = self.freq_map.get_mut(&old_val) {
            *freq -=1;
            if *freq == 0 {self.freq_map.remove(&old_val); }
        }
        let  new_value = old_val + val;
        self.nums2[idx] = new_value;
        *self.freq_map.entry(new_value).or_insert(0) +=1;
    }
    
    fn count(&mut self, tot: i32) -> i32 {
        let mut counter: i32 = 0; 
        for &i in &self.nums1 {
            if i >= tot {
                break;
            }
            let complement = tot - i;

            if self.freq_map.contains_key(&complement) {
                if let Some(&sum_freq) = self.freq_map.get(&complement) {
                    counter += sum_freq;
                }
                
            }
            

        }
        println!("{:?}",counter);
        counter
    }
}

//I need a fn apart because de index is important
pub fn update_map(v : &Vec<i32>)-> HashMap<i32, i32> {
    let mut map :HashMap<i32, i32> = HashMap::new();
    for &n in v  {
            if map.contains_key(&n) {
                if let Some(&number) =map.get(&n){
                    map.insert(n, number + 1);
                };
            }else {
                map.insert(n, 1);
            }
        }
    map
}
