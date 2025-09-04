
struct Solution;
fn main() {
    let s = Solution::minimum_operations(vec![1,2,3,4,2,3,3,5,7]);
    println!("{:?}", s);
    let s = Solution::minimum_operations(vec![4,5,6,4,4]);
    println!("{:?}", s); 
    let s = Solution::minimum_operations(vec![6,7,8,9]);
    println!("{:?}", s);
}

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut operations = 0;
        let mut are_uniqued = false;
        let mut index = 0;
    'principal: while !are_uniqued {
            let mut hashmap : std::collections::HashMap<i32 , i32> = HashMap::new();
            for i in index .. nums.len()  {
                if hashmap.contains_key(&nums[i]) {
                    //Break 
                    index += 3;
                    operations +=1;
                    continue 'principal;
                }
                *hashmap.entry(nums[i]).or_insert(1) +=1; 
            }
            are_uniqued = true;
        }
        operations
    }
}