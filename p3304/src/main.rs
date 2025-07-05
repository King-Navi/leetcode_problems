    // radix 36 or UTF-8, i choose UTF-8
    // let t1 = 'a' as  u32;
    // let t2 = 'z' as u32;
    // println!("{:?}, {:?}", t1, t2);
    // let y1 =char::from_u32(t1)
    //     .expect("Panic"); 
    // print!("{}", y1);
fn main() {
    let sol = Solution::kth_character(10);
    println!("{}",sol);
    
}
pub struct Solution;
const SUPERIOR_LIMIT : u32= 122;
const INFERIOR_LIMIT : u32 = 97;

impl Solution {
    pub fn kth_character(k: i32) -> char {
        let result =alice('a'.to_string(), k as usize);
        if let Some(c)  =  result.chars().nth(k as usize -1) {
            return c;
        }
        'a'
    }

}

pub fn alice(strs:String, k:  usize)-> String{
    let len = strs.chars().count();
    if k <= len{
        return strs;
    }
    let mut new_string = String::new();
    for c in strs.chars(){
        new_string.push(convertor_to_chat(convertor_to_int(c)));
    }
    let mut strs2: String = strs.clone();
    strs2.push_str(&new_string);
    
    alice(strs2, k)
}

pub fn convertor_to_int(c : char)->u32{
    let number = c as u32 + 1;
    if number > SUPERIOR_LIMIT {
        return INFERIOR_LIMIT;
    } 
    number
}


pub fn convertor_to_chat(integer : u32)->char{
    let char2 = char::from_u32(integer)
        .expect("No valid UTF-8");

    return char2;
}