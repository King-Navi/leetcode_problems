struct Solution;
fn main() {
    let s = Solution::has_match("leetcode".to_string(), "ee*e".to_string());
    println!("{:?}", s);
    let s = Solution::has_match("xks".to_string(), "s*".to_string());
    println!("{:?}", s);
}
impl Solution {
    pub fn has_match(s: String, p: String) -> bool {
        let mut result = false;
        let s_vec: Vec<char> = s.chars().collect();
        let p_vec: Vec<char> = p.chars().collect();
        let mut index_of_p = 0;
        let mut max_index_p = p_vec.len() - 1;
        let mut index_second_part_p = 0;
        let mut second_part_p = false;
        for (i, &c) in s_vec.iter().enumerate() {
            if index_of_p <= max_index_p && '*' == p_vec[index_of_p] {
                index_of_p += 1;
                index_second_part_p = index_of_p;
                second_part_p = true;
            }

            if index_of_p <= max_index_p && c == p_vec[index_of_p] {
                index_of_p += 1;
            } else {
                //need to check again the first token of p
                if second_part_p {
                    index_of_p = index_second_part_p;
                } else {
                    index_of_p = 0;
                }
            }
            if index_of_p >= max_index_p {
                result = true;
            }
        }

        result
    }
}
