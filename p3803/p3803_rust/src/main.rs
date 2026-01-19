struct Solution;

fn main() {
    println!("{}", Solution::residue_prefixes(String::from("abc")));

    println!("{}", Solution::residue_prefixes(String::from("dd")));

    println!("{}", Solution::residue_prefixes(String::from("bob")));

    println!("{}", Solution::residue_prefixes(String::from("kl")));

    println!("{}", Solution::residue_prefixes(String::from("fhhi")));

    //assert_eq!(Solution::residue_prefixes(String::from("abc")), 2);
    //assert_eq!(Solution::residue_prefixes(String::from("dd")), 1);
    //assert_eq!(Solution::residue_prefixes(String::from("bob")), 2);
    //assert_eq!(Solution::residue_prefixes(String::from("kl")), 2);
    //assert_eq!(Solution::residue_prefixes(String::from("fhhi")), 2, "Caso 5 fallo");
}

impl Solution {
    
    pub fn residue_prefixes(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut counter: i32 =0;
        for n in 1..=chars.len() {
            counter += evaluate_condition(&chars[..n]);
        }
        counter
    }
}

fn evaluate_condition(slice: &[char]) -> i32 {
    use std::collections::HashMap;
    let mut hashmap: HashMap<char, i32> = HashMap::new();
    for n in 0..slice.len() {
        if hashmap.contains_key(&slice[n]) {
            continue;
        } else {
            *hashmap.entry(slice[n]).or_insert(0) += 1;
        }
    }
    if (slice.len() % 3 ) == hashmap.keys().count() {
        return 1;
    }
    0
}
