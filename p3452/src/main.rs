struct Solution;

fn main() {
    let  s = Solution::sum_of_good_numbers(vec![1,3,2,1,5,4], 2);
    println!("{s}");
}
impl Solution {
    pub fn sum_of_good_numbers(nums: Vec<i32>, k: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut sum = 0;
        for (i , n) in nums.iter().enumerate() {
            let mut first_condition = false;
            let mut second_condition = false;
            if  i as isize - k as isize >= 0 &&
                let Some(number_first) = nums.get(i - k as usize)  {
                if number_first < n {
                    first_condition = true;
                }
            }else {
                //NO exite uno     VALIDO
                first_condition = true;
            }

            if i as isize + k as isize >= 0 &&
                let Some(number_second) = nums.get(i + k as usize)  {
                if number_second < n {
                    second_condition = true;
                }
            }else {
                //NO exite el otro    VALIDO SI EL UNO NO EXITE
                second_condition = true;
            }

            if first_condition && second_condition {
                sum += n;
            }
        }
        sum
    }
}
