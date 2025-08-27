struct Solution;
fn main() {
    println!("gcd([2,2])   = {}", gcd(&[2,2]));
    println!("lcm([2,2])   = {}", lcm(&[2,2]));
    println!("prod([2,2])  = {}", prod(&[2,2]));
    println!("holds([2,2]) = {}", holds(&[2,2])); 

    let s = Solution::max_length(vec![1, 2, 1, 2, 1, 1, 1]);
    println!("max_length([1,2,1,2,1,1,1]) = {}", s);

    println!("max_length([2,2])        = {}", Solution::max_length(vec![2,2]));        // 2
    println!("max_length([2,1,1,1])    = {}", Solution::max_length(vec![2,1,1,1]));// 4
    println!("max_length([2,2,1])      = {}", Solution::max_length(vec![2,2,1]));  // 1
    println!("max_length([1,1,1])      = {}", Solution::max_length(vec![1,1,1]));  // 3
}

impl Solution {
    pub fn max_length(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 { return 0; }
        for i in 0..n {
            let slice = &nums[i..];
            if holds(slice) {
                return (n - i) as i32; 
            }
        }
        0
    }
}

fn holds(slice: &[i32]) -> bool {
    if slice.is_empty() { return false; }
    let g = gcd(slice) as i128;
    let l = lcm(slice) as i128;
    let p = prod(slice);
    p == l * g
}

pub fn prod(list: &[i32]) -> i128 {
    let mut p: i128 = 1;
    for &x in list {
        p *= x as i128;
    }
    p
}

fn gcd_two(mut a: i64, mut b: i64) -> i64 {
    a = a.abs(); b = b.abs();
    if a == 0 { return b; }
    if b == 0 { return a; }
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

fn gcd(list: &[i32]) -> i64 {
    match list {
        [] => 0,
        [x] => (*x as i64).abs(),
        [a, rest @ ..] => {
            let mut g = gcd_two(*a as i64, rest[0] as i64);
            for &x in &rest[1..] {
                g = gcd_two(g, x as i64);
            }
            g
        }
    }
}

fn lcm_two(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 { return 0; }
    let g = gcd_two(a, b) as i128;
    let res = ((a as i128 / g) * (b as i128)).abs();
    res as i64
}

fn lcm(list: &[i32]) -> i64 {
    match list {
        [] => 0,
        [x] => (*x as i64).abs(),
        [a, rest @ ..] => {
            let mut acc = lcm_two(*a as i64, rest[0] as i64);
            for &x in &rest[1..] {
                acc = lcm_two(acc, x as i64);
                if acc == 0 { break; }
            }
            acc
        }
    }
}
