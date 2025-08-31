struct Solution;
fn main() {
    let s = Solution::has_match("leetcode".to_string(), "ee*e".to_string());
    println!("{:?}", s); //true
    let s = Solution::has_match("xks".to_string(), "s*".to_string());
    println!("{:?}", s); //true
    let s = Solution::has_match("car".to_string(), "c*v".to_string());
    println!("{:?}", s); //false
    let s = Solution::has_match("l".to_string(), "*".to_string());
    println!("{:?}", s); //true
    let s = Solution::has_match("mlmww".to_string(), "ml*wl".to_string());
    println!("{:?}", s); //false
}
impl Solution {
    // pub fn has_match(s: String, p: String) -> bool {
    //     let mut result = false;
    //     let s_vec: Vec<char> = s.chars().collect();
    //     let p_vec: Vec<char> = p.chars().collect();
    //     let mut index_of_p = 0;
    //     let mut max_index_p = p_vec.len() - 1;
    //     let mut index_second_part_p = 0;
    //     let mut second_part_p = false;
    //     if p_vec.len() == 1 && '*' == p_vec[0]  {
    //         return true;
    //     }
    //     for (i, &c) in s_vec.iter().enumerate() {

    //         if index_of_p <= max_index_p && '*' == p_vec[index_of_p] {
    //             index_of_p += 1;
    //             index_second_part_p = index_of_p;
    //             second_part_p = true;
    //         }

    //         if index_of_p <= max_index_p
    //             && c == p_vec[index_of_p] {

    //         } else {
    //             if second_part_p {
    //                 index_of_p = index_second_part_p;
    //                 continue;
    //             } else {
    //                 index_of_p = 0;
    //                 continue;
    //             }
    //         }

    //         if index_of_p >= max_index_p{
    //             result = true;
    //             break;
    //         }
    //         index_of_p += 1;
    //         if s_vec.len() -1 == i {
    //             return true;
    //         }

    //     }

    //     result
    // }
    pub fn has_match(s: String, p: String) -> bool {
        let mut result = false;
        let mut first_part: Vec<char> = vec![];
        let mut second_part: Vec<char> = vec![];
        {
            let mut flag_second_part = false;
            for c in p.chars() {
                if c == '*' {
                    flag_second_part = true;
                }
                if !flag_second_part {
                    first_part.push(c);
                }
                if flag_second_part {
                    second_part.push(c);
                }
            }
        }
        {
            let mut flag_second_part = false;
            let index = 0;
            for c in s.chars() {
                
            }
        }

        result
    }
}
