use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn min_distinct_freq_pair(nums: Vec<i32>) -> Vec<i32> {
        let max_lenght = nums.len();
        if  max_lenght == 1 || max_lenght == 2 {
            return  vec![-1,-1];
        }
        let mut hashmap :HashMap<i32,i32>= HashMap::new();
        for n in nums {
            *hashmap.entry(n).or_insert(0) += 1;
        }
        let mut v: Vec<(i32, i32)> = hashmap.into_iter().collect();
        let max_lenght = v.len();
        if  max_lenght == 1 {
            return  vec![-1,-1];
        }
        v.sort_by(|(k1, f1), (k2, f2)| {
            k1.cmp(k2).then_with(|| f1.cmp(f2))
        });
        println!("{v:?}");
        
        let (x, frecuency_x) = v.iter().skip(0).next().unwrap();
        let &(mut y, mut frecuency_y) = v.iter().skip(1).next().unwrap();
        let mut amount = 0;
        
        while x == &y || frecuency_x == &frecuency_y   {
            if max_lenght <= amount {
                return vec![-1,-1];
            }
            (y, frecuency_y) = *v.iter().skip(amount).next().unwrap();
            amount +=1;
        }
        
        vec![*x,y]
    }
}

fn main() {
    println!("{:?}", Solution::min_distinct_freq_pair(vec![1,1,2,2,3,4]));
    println!("{:?}", Solution::min_distinct_freq_pair(vec![3,5,4]));
    println!("{:?}", Solution::min_distinct_freq_pair(vec![5,5,4]));
}