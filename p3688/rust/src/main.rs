struct Solution;
impl Solution {
    pub fn even_number_bitwise_o_rs(nums: Vec<i32>) -> i32 {
        if nums.len()== 1 {
            if nums[0] % 2 == 0 {
                return nums[0];
            }else {
                return 0;
            }
        }
        let mut max_len = 0;
        let mut vec_even = vec![]; 
        for n in nums {
            let s = format!("{n:b}");
            if s.len() > max_len {
                max_len = s.len();
            }
            if n % 2 == 0 {
                //even
                vec_even.push(n);   
            }
        }
        let mut vec_even_bit:Vec<Vec<char>> = vec![];
        for n in &vec_even {
            let mut s = format!("{n:b}").chars().collect::<Vec<char>>();
            let mut missing_sapce = 0;
            if s.len() < max_len {
                missing_sapce = max_len - s.len();
            }
            for _ in 0..missing_sapce {
                s.insert(0,'0');
            }
            
            vec_even_bit.push(s);
        }
        if vec_even_bit.is_empty() {
            return  0;
        }
        let result = compare(vec_even_bit)[0].clone();
        
        let mut str = String::new();
        for c in result {
            str.push(c);
        }
        i32::from_str_radix(&str,2).unwrap()
    }
}

fn compare(vec: Vec<Vec<char>>) -> Vec<Vec<char>> {
    if vec.len() <= 1 {
        return vec;
    }
    let mut result: Vec<Vec<char>> = vec![];
    let first:Vec<char>=  vec[0].clone();
    let second:Vec<char>= vec[1].clone(); 
    let mut new:Vec<char> = vec![];
    for (i, u) in first.iter().enumerate() {
        if first[i].to_digit(10).unwrap() == 1 
            || second[i].to_digit(10).unwrap() == 1  {
            new.push('1');
        }else {
            new.push('0');
        }
    }
    result.push(new);
    for index in 2..vec.len() {
        if index >= vec.len() {
            break;
        }
        result.push(vec[index].clone());    
    }
    
    return compare(result);
}
fn main() {
    //Solution::even_number_bitwise_o_rs(vec![1,2,3,4,5,6]);
    println!("{:?}", Solution::even_number_bitwise_o_rs(vec![1,2,3,4,5,6]));
    println!("{:?}", Solution::even_number_bitwise_o_rs(vec![7,9,11]));
    println!("{:?}", Solution::even_number_bitwise_o_rs(vec![1,8,16]));
    
}
