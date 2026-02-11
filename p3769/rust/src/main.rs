struct Solution;
impl Solution {
pub fn sort_by_reflection(nums: Vec<i32>) -> Vec<i32> {
    let mut v: Vec<(i32, i32)> = vec![];

    for &n in nums.iter() {
        let bin = format!("{n:b}");
        let rev: String = bin.chars().rev().collect();
        let key = i32::from_str_radix(&rev, 2).unwrap();
        v.push((key, n));
    }

    v.sort_by(|a, b| {
        a.0.cmp(&b.0).then(a.1.cmp(&b.1))
    });

    v.into_iter().map(|(_, n)| n).collect()
}


}

fn main() {
    println!("Hello, world!");
}
