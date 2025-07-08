
pub  struct Solution;
fn main() {
    Solution::find_lucky(
        vec![2,2,3,4]
    );
}

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut lucky_numbers: HashSet<i32> = HashSet::new() ;
        let mut map: HashMap<i32,i32> = HashMap::new();
        for &actual_value in arr.iter() {
            if map.contains_key(&actual_value) {
                
               if let Some(value_of_map) = map.get(&actual_value){
                let new_value = value_of_map +1; 
                map.insert(actual_value, value_of_map + 1 );

                if new_value == actual_value {
                    lucky_numbers.insert(actual_value);
                }else 

                if lucky_numbers.contains(&actual_value)  {
                    lucky_numbers.remove(&actual_value);
                }
                
               };
            }else {
                map.insert(actual_value, 1);
                if actual_value == 1 {
                    lucky_numbers.insert(1);
                }
            }
        }
        if lucky_numbers.is_empty() {
            -1
        }else {
            match lucky_numbers.iter().max() {
                Some(&value) => value,
                None => -1
            }
        }
    }
}


