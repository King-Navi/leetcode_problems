struct Solution;
fn main() {
    println!("Hello, world!");
}
impl Solution {
    pub fn count_partitions(nums: Vec<i32>) -> i32 {
        let mut sum_part_1 = 0;
        let lenght = nums.len() -1;
        let mut result = 0;
        for (i, &n) in nums.iter().enumerate() {
            if i == lenght {
                break;
            }
            sum_part_1 += n;
            let mut minus_total = 0;
            let start = i + 1;
            let stop = lenght + 1;
            for s in &nums[start .. stop] {
                minus_total += -s;
            }
            let var_temp = sum_part_1 - minus_total;
            if var_temp % 2 == 0{
                result +=1;
            }
        }
        result

    }
}