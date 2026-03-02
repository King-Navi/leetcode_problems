
struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn toggle_light_bulbs(bulbs: Vec<i32>) -> Vec<i32> {
        let mut hashmap : HashMap<i32, bool>= HashMap::new();
        for n in bulbs {
            if hashmap.contains_key(&n) {
                let num = hashmap.get_mut(&n).unwrap();
                
                if *num {
                    *num = false;
                    //need to turn on
                }else {
                    //need to turn off
                    *num = true;
                }
            }else {
                *hashmap.entry(n).or_insert(true);
            }
            
        }
        let mut result = vec![];
        for (k,v) in hashmap {
            if v {
                result.push(k);
            }
        }
        result.sort();
        result
    }
}
fn main() {
    println!("Hello, world!");
}
