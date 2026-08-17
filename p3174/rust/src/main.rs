struct Solution;
impl Solution {
    pub fn clear_digits(s: String) -> String {
        let mut vec = s.chars().collect::<Vec<char>>();
        'maiin: loop {
            for i in (0..vec.len()).rev() {
                if vec[i].is_numeric() {
                    let post_digit= i;
                    for j in (0..post_digit).rev() {
                        if vec[j].is_alphabetic() {
                            vec.remove(j);
                            vec.remove(i-1);
                            continue 'maiin;
                        }else if vec[j].is_numeric() && j == 0{
                            break 'maiin;
                        }
                    }
                }
            }
            break ;
        }       
        vec.iter().collect::<String>()
    }
}
fn main() {
    println!("{}",Solution::clear_digits("abc".to_string()));
    println!("{}",Solution::clear_digits("ab12".to_string()));

}
