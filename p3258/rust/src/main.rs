struct Solution;
impl Solution {
    pub fn count_k_constraint_substrings(s: String, k: i32) -> i32 {
        let vec = s.chars().collect::<Vec<char>>();
        let mut counter = 0;
        for start in 0..vec.len() {
            for end in start+1..vec.len()+1 {
                if checkn_condition(&vec[start..end], k){
                    counter += 1;
                }
            }
        }
        counter
    }
}
fn checkn_condition(s:&[char], k: i32) -> bool{
    if s.is_empty() {
        return false;
    }
    let mut ones = 0;
    let mut ceros = 0;

    for &c in s {
        if '1' == c {
            ones += 1;
        } else {
            ceros += 1;
        }
    }
    ones <= k || ceros <= k 
}

fn main() {
    println!("{}", Solution::count_k_constraint_substrings("10101".to_string(), 1));
}
/*
You are given a binary string s and an integer k.

A binary string satisfies the k-constraint if either of the following conditions holds:

The number of 0's in the string is at most k.
The number of 1's in the string is at most k.
Return an integer denoting the number of substrings of s that satisfy the k-constraint.

 

Example 1:

Input: s = "10101", k = 1

Output: 12

Explanation:

Every substring of s except the substrings "1010", "10101", and "0101" satisfies the k-constraint.

Example 2:

Input: s = "1010101", k = 2

Output: 25

Explanation:

Every substring of s except the substrings with a length greater than 5 satisfies the k-constraint.

Example 3:

Input: s = "11111", k = 1

Output: 15

Explanation:

All substrings of s satisfy the k-constraint.

 

Constraints:

1 <= s.length <= 50 
1 <= k <= s.length
s[i] is either '0' or '1'.
*/