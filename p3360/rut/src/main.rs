struct Solution;
impl Solution {
    pub fn can_alice_win(n: i32) -> bool {
        let mut left : i32= n;
        let mut start = 10;
        let mut wins =false;
        loop {
            left = left - start;
            if left.is_negative() {
                break;    
            }
            wins = !wins;
            start -= 1;
        }
        wins
    }
}
fn main() {
    println!("{}",Solution::can_alice_win(12));
    println!("{}",Solution::can_alice_win(1));
}
