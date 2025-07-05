pub struct Solution;

fn main() {
    let mut a = vec![0,1,2,2,3,0,4,2]
;
    
    println!("{}", Solution::remove_element(& mut a, 2))
}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut counter : i32 = 0;
        let mut ceros: Vec<i32> = vec![];
        let mut aux_evc : Vec<i32> = vec![];
        for &n in nums.iter(){
            if n == val{
                ceros.push(0);
            }else {
                counter += 1;
                aux_evc.push(n);
            }
        }
        nums.clear();
        println!("{:?}", aux_evc);
        println!("{:?}", ceros);
        nums.append(&mut aux_evc);
        nums.append(&mut ceros);
        println!("{:?}", nums);
        counter
    }
}