struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut has: HashSet<i32> = HashSet::new();
        rrr(n, &mut has)
    }
}

pub fn rrr(n: i32, has:&mut HashSet<i32>) -> bool {
    if n == 1 {
        return true;
    }

    let str = n.to_string().chars().collect::<Vec<char>>();
    //MAKE THE POWER
    let mut sum: i32 = 0;
    for c in str {
        let n = c.to_digit(10).unwrap() as i32;
        sum += n * n;
        print!("{}", n);
    }
    if has.contains(&sum) {
        return false;
    }else {
        has.insert(sum);
    }
    

    rrr(sum, has)
}
fn main() {
    //println!("{}", Solution::is_happy(19));
    println!("{}", Solution::is_happy(2));
    //println!("{}", Solution::is_happy(7));
}
