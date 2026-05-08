struct Solution;

impl Solution {
    pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
        let mut original_nums = nums.clone();
        let n = nums.len();
        let mut result = 0_i32;
        if let Some(indices_cero) = have_cero(&original_nums) {
            for &curr in &indices_cero {
                for &dir_inicial in &[false, true] {
                    let mut nums_simulacion = original_nums.clone();
                    let mut direccion = dir_inicial;
                    let mut index = curr as i32;

                    'main: loop {
                        if index < 0 || index >= n as i32 {
                            break 'main;
                        }

                        let idx = index as usize;

                        if nums_simulacion[idx] == 0 {
                            if direccion {
                                index += 1;
                            } else {
                                index -= 1;
                            }
                        } else {
                            nums_simulacion[idx] -= 1;
                            direccion = !direccion;
                            if direccion {
                                index += 1;
                            } else {
                                index -= 1;
                            }
                        }
                    }

                    if nums_simulacion.iter().all(|&x| x == 0) {
                        result += 1;
                    }
                }
            }
        }
        result
    }
}
pub fn have_cero(nums: &[i32]) -> Option<Vec<usize>> {
    let mut curr: Option<Vec<usize>> = Some(Vec::new());
    for (i, n) in nums.iter().enumerate() {
        if *n == 0 {
            curr.get_or_insert(Vec::new()).push(i);
        }
    }
    return curr;
}
fn main() {
    println!(
        "{:?}",
        Solution::count_valid_selections(vec![1, 0, 2, 0, 3])
    );
}
