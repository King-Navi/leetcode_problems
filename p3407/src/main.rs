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
    let s = Solution::has_match("tokk".to_string(), "t*t".to_string());
    println!("{:?}", s); //false
    let s = Solution::has_match("wwmwww".to_string(), "wm*".to_string());
    println!("{:?}", s); //true
    let s = Solution::has_match("ggmgmrmrm".to_string(), "*gmg".to_string());
    println!("{:?}", s); //true
    let s = Solution::has_match("mqmmmqmqbmqq".to_string(), "mmqm*b".to_string());
    println!("{:?}", s); //true

    // let s: Vec<char> = "leetcode".chars().collect();
    // let ss: Vec<char> = "ee".chars().collect();
    // let res = has_substring(&s,& ss);
    // println!("{:?}", res);
}
impl Solution {
    pub fn has_match(s: String, p: String) -> bool {
        if p.len() == 1 && p.chars().collect::<Vec<char>>()[0] == '*' {
            return true;
        }
        let mut first_part: Vec<char> = vec![];
        let mut second_part: Vec<char> = vec![];
        {
            let mut flag_second_part = false;
            for c in p.chars() {
                if c == '*' {
                    flag_second_part = true;
                    continue;
                }
                if !flag_second_part {
                    first_part.push(c);
                }
                if flag_second_part {
                    second_part.push(c);
                }
            }
        }
        let s_chars : Vec<char> = s.chars().collect();
        let index_second_part = has_substring(&s_chars, &first_part);
        print!("{:?} , ", index_second_part);
        if index_second_part < 0 {
            return false;
        }
        let last_result = has_substring(&s_chars[index_second_part as usize .. ], &second_part);
        println!("{:?}", last_result);
        if last_result < 0 {
            return false;
        }
        true
    }
}



pub fn has_substring(main: &[char], substring : &[char] ) -> isize {
    if substring.is_empty() {
        return 0;
    }
    for (i, _) in main.iter().enumerate() {
        let mut index_copy = i;
        let mut match_result = true;
        let mut saved_j = 0;
        for j in 0 .. substring.len() {
            if index_copy >= main.len() {
                match_result = false;
                break;
            }
            if  substring[j] != main[index_copy] {
                match_result = false;
                break;
            } 
            index_copy += 1;
            saved_j = j + i;
        }
        if match_result {
            //Si hay coincidencia
            return saved_j as isize + 1;
        }
    }
    -1
}


/*
pub fn has_match(s: String, p: String) -> bool {
        if p.len() == 1 && p.chars().collect::<Vec<char>>()[0] == '*' {
            return true;
        }
        let mut first_part: Vec<char> = vec![];
        let mut second_part: Vec<char> = vec![];
        {
            let mut flag_second_part = false;
            for c in p.chars() {
                if c == '*' {
                    flag_second_part = true;
                    continue;
                }
                if !flag_second_part {
                    first_part.push(c);
                }
                if flag_second_part {
                    second_part.push(c);
                }
            }
        }
        let s_chars : Vec<char> = s.chars().collect();
        let mut index_s_save = 0;
        'first: {
            if first_part.is_empty() {
                break 'first;
            }
            let mut index_first_part: isize = -1;
            for (i , &c) in s_chars.iter().enumerate() {
                index_s_save = i;
                index_first_part +=1;
                if  index_first_part >= 0
                    && c == first_part[index_first_part as usize] 
                    && (index_first_part as usize) < first_part.len() {
                    //continue;
                    if index_first_part == first_part.len() as isize - 1 {
                        index_s_save +=1;
                        break;
                    }
                } else {
                    if c == first_part[0] {
                        index_first_part = 0;
                    }else {
                       index_first_part = -1;  
                    }
                    
                }
            }
            if index_first_part != first_part.len() as isize - 1 {
                return false;
            }
        }
        'second : {
            if second_part.is_empty() {
                break 'second;
            }
            let mut index_second_part: isize = -1;
            for i in index_s_save..s_chars.len() {
                
                index_second_part +=1;
                if  index_second_part >= 0
                    && s_chars[i] == second_part[index_second_part as usize] 
                    && (index_second_part as usize) < second_part.len() {
                    //continue;
                    if index_second_part == second_part.len() as isize - 1 {
                        break;
                    }
                } else {
                    if s_chars[i] == second_part[0] {
                        index_second_part = 0;
                    } else {
                      index_second_part = -1;   
                    }
                }
            }
            if index_second_part != second_part.len() as isize - 1 {
                return  false;
            }
        }

        true
    }

*/