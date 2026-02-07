struct Solution;
impl Solution {
    pub fn mirror_distance(n: i32) -> i32 {
        (n - reverse_int(n)).abs()
    }
}

pub fn reverse_int(n : i32) -> i32{
    let mut str_int = n.to_string().chars().collect::<Vec<char>>();
    str_int.reverse();
    let mut new_int_in_str = String::new();
    for c in str_int {
        new_int_in_str.push(c);
    }
    println!("{:?}",new_int_in_str);
    new_int_in_str.parse::<i32>().unwrap()
}
fn main() {
    println!("{}", reverse_int(321));
}
