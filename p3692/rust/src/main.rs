struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn majority_frequency_group(s: String) -> String {
        let mut hashmp: HashMap<char, i32> = HashMap::new();
        for c in s.chars() {
            *hashmp.entry(c).or_insert(0) += 1;
        }
        let mut resul: HashMap<i32, Vec<char>> = HashMap::new();
        loop {
            if let Some((k, v)) = hashmp.iter().max_by(|(k, v), (k2, v2)| v.cmp(v2)) {
                let mut max_frecuency = *v;
                let key = *k;
                hashmp.remove(&key);
                

                if resul.contains_key(&max_frecuency) {
                    let vect = resul.get_mut(&max_frecuency).unwrap();
                    vect.push(key);
                }else {
                    resul.insert(max_frecuency, vec![key]);
                }
            }else {
                break;
            }
        }
        let (k, v) = resul.iter().max_by(
            |(k, v), (k2, v2)| {
                v.len().cmp(&v2.len()).then_with(|| k.cmp(k2))
            }).unwrap() ;
        
        let mut result = String::new();
        for c in v {
            result.push(*c);
        }
        result
    }
}
fn main() {
    println!(
        "{:?}",
        Solution::majority_frequency_group(String::from("aaabbbccdddde"))
    );
}
