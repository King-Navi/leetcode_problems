pub struct Solution;
fn main() {
    //  let mut v = vec![];
    //  calcuate_bin(2197, &mut v);
    
    //let v = calculate_hex(2);
    
    //let v = Solution::concat_hex36(1);
    
    // let mut v = vec![];
    // calcuate_base_36(8, &mut v);

    let v  = hexatrigesimal(8);
    println!("{:?}", v);
}

impl Solution {
    pub fn concat_hex36(n: i32) -> String {
        let n_squared = n * n;
        let n_cubed = n * n * n;
        let n_squared_str = calculate_hex(n_squared);
        let n_cubed_str = hexatrigesimal(n_cubed);
        let mut result = String::new();
        result.push_str(&n_squared_str);
        result.push_str(&n_cubed_str);
        result
    }
}

pub fn hexatrigesimal(n: i32) -> String {
    let mut v = vec![];
    calcuate_base_36(n, &mut v);
    let mut result = String::new();
    for i in &v {
        let digit = *i as usize;
        let hex = get_char_36(&digit);
        result.push(hex);
    }
    result
        .chars()
        .rev()
        .collect::<String>()
        .trim_start_matches('0')
        .to_string()
}


pub fn calculate_hex(n: i32) -> String {
    let mut v = vec![];
    calcuate_bin(n, &mut v);
    {
        let leng = v.len() % 4;
        if leng != 0 {
            let iterations = 4 - leng;
            for _ in 0..iterations {
                v.push(0);
            }
        }
    }
    let mut length_actual_begin: usize = 0;
    let mut length_actual: usize = 4;
    let mut result = String::new();

    while &length_actual <= &v.len() {
        let hex = calcular_hex(&v[length_actual_begin..length_actual]);
        result.push(hex);
        length_actual += 4;
        length_actual_begin += 4;
    }

    return result.chars().rev().collect();
}

pub fn calcuate_bin(n: i32, vec_bin: &mut Vec<u8>) {
    if n <= 1 {
        vec_bin.push(n as u8);
        //The vec isn't reversed because i think this way it's more appropiate form our goal
        return;
    }
    let mod_ope = n % 2;
    vec_bin.push(mod_ope as u8);
    calcuate_bin(n / 2, vec_bin);
}

pub fn calcular_hex(s: &[u8]) -> char {
    let mut number: usize = 0;
    if s[3] == 1 {
        number += 8;
    }
    if s[2] == 1 {
        number += 4;
    }
    if s[1] == 1 {
        number += 2;
    }
    if s[0] == 1 {
        number += 1;
    }
    if let Some(c) = get_value(&number) {
        return c;
    }
    'G'
}



pub fn calcuate_base_36(n: i32, vec_bin: &mut Vec<u32>) {
    if n <= 1 {
        vec_bin.push(n as u32);
        //The vec isn't reversed because i think this way it's more appropiate form our goal
        return;
    }
    let mod_ope = n % 36;
    vec_bin.push(mod_ope as u32);
    calcuate_base_36(n / 36, vec_bin);
}


pub fn get_char_36(n : &usize) -> char {
    if let Some(c) = get_value(&n) {
        return c;
    }
    'Ñ'
}

fn get_value(key: &usize) -> Option<char> {
    match key {
        0 => Some('0'),
        1 => Some('1'),
        2 => Some('2'),
        3 => Some('3'),
        4 => Some('4'),
        5 => Some('5'),
        6 => Some('6'),
        7 => Some('7'),
        8 => Some('8'),
        9 => Some('9'),
        10 => Some('A'),
        11 => Some('B'),
        12 => Some('C'),
        13 => Some('D'),
        14 => Some('E'),
        15 => Some('F'),
        16 => Some('G'),
        17 => Some('H'),
        18 => Some('I'),
        19 => Some('J'),
        20 => Some('K'),
        21 => Some('L'),
        22 => Some('M'),
        23 => Some('N'),
        24 => Some('O'),
        25 => Some('P'),
        26 => Some('Q'),
        27 => Some('R'),
        28 => Some('S'),
        29 => Some('T'),
        30 => Some('U'),
        31 => Some('V'),
        32 => Some('W'),
        33 => Some('X'),
        34 => Some('Y'),
        35 => Some('Z'),
        _ => None,
    }
}
