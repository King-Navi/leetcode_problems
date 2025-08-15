struct Solution;
fn main() {
    let b =Solution::has_special_substring("jjbbbaaf".to_string(), 1);
    println!("{b}");
}

impl Solution {
    pub fn has_special_substring(s: String, k: i32) -> bool {
        if k == 1 && s.len() == 1 {
            return  true;
        }
        let chars : Vec<char> = s.chars().collect();
        let mut i = 0; 
        'outer: while i < chars.len() {
            if i > 0 
                && let Some(&before) = chars.get(i -1) {
                //Exits
                if before == chars[i] {
                    i += 1;
                    continue 'outer;
                }
                let stop = k as usize +i;
                for j in i..stop {
                    if let Some(&next_char) = chars.get(j) {
                        if next_char != chars[i]  {
                            i += 1;
                            continue 'outer;
                        }
                    }else {
                        return false;
                    }
                }
                if let Some(&last) = chars.get(stop) {
                    if last != chars[i] {
                        return true;
                    }
                }else {
                    return true;
                }
            }else {
            //     //No E
                let stop = k as usize +i;
                for j in i..stop {
                    if let Some(&next_char) = chars.get(j) {
                        if next_char != chars[i]  {
                            i += 1;
                            continue 'outer;
                        }
                    }else {
                        return false;
                    }
                }
                if let Some(&last) = chars.get(stop) {
                    if last != chars[i] {
                        return true;
                    }
                }else {
                    return true;
                }
                
            }
            i +=1;
        }
        false
    }
}