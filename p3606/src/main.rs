pub struct Solution;
fn main() {
    Solution::validate_coupons(
        vec![
            "SAVE20".to_string(),
            "".to_string(),
            "PHARMA5".to_string(),
            "SAVE@20".to_string(),
        ],
        vec![
            "restaurant".to_string(),
            "grocery".to_string(),
            "pharmacy".to_string(),
            "restaurant".to_string(),
        ],
        vec![true, true, true, true],
    );
}

/*

businessLine[i] is one of the following four categories:
-"electronics",
-"grocery",
-"pharmacy",
-"restaurant".

*/
impl Solution {
    pub fn validate_coupons(
        code: Vec<String>,
        business_line: Vec<String>,
        is_active: Vec<bool>,
    ) -> Vec<String> {
        let mut electronics_valid_code: Vec<String> = vec![];
        let mut grocery_valid_code: Vec<String> = vec![];
        let mut pharmacy_valid_code: Vec<String> = vec![];
        let mut restaurant_valid_code: Vec<String> = vec![];
        {
            for idx in (0..is_active.len()).rev() {
                if is_active[idx] && valid_string(&code[idx]) {
                    match business_line[idx].as_str() {
                        "electronics" => electronics_valid_code.push(code[idx].clone()),
                        "grocery"     => grocery_valid_code.push(code[idx].clone()),
                        "pharmacy"    => pharmacy_valid_code.push(code[idx].clone()),
                        "restaurant"  => restaurant_valid_code.push(code[idx].clone()),
                        _ => {}
                    }
                }
            }
            electronics_valid_code.sort();
            grocery_valid_code.sort();
            pharmacy_valid_code.sort();
            restaurant_valid_code.sort();
        }
        
        electronics_valid_code.append(&mut grocery_valid_code);
        pharmacy_valid_code.append(&mut restaurant_valid_code);

        electronics_valid_code.append(&mut pharmacy_valid_code);
        println!("{:?}", electronics_valid_code);
        electronics_valid_code
        
    }
}

pub fn valid_string(chars: &String) -> bool {
    if chars.is_empty() {
        return false;
    }

    chars
        .chars()
        .all(|c| 
            (c >= 'a' && c <= 'z') 
            || (c >= 'A' && c <= 'Z') 
            || (c >= '0' && c <= '9') 
            || c == '_')
}
