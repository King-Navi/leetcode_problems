pub struct Solution;
fn main() {
    Solution::count_hill_valley(vec![2, 4, 1, 1, 6, 5]);
}

impl Solution {
    pub fn count_hill_valley(nums: Vec<i32>) -> i32 {
        let mut counter_result = 0;

        let last_index: usize = nums.iter().count() - 1;

        for (i, &n) in nums.iter().enumerate() {
            if i == 0 {
                //first number never a hill or valley
                continue;
            }
            if nums[i] == nums[i -1] {
                //same number no sense to check
                continue;
            }

            if i == last_index {
                //last number never a hill or valley
                break;
            }

            //Only if isn't outboundindex
            {
                //hill
                let mut behind_number: Option<i32> = None;
                let mut front_number: Option<i32> = None;
                {
                    let mut index_ = i +1;
                    while true {
                        if nums[index_] < n {
                            front_number = Some(nums[index_]);
                            break;
                        }
                        if nums[index_] > n  {
                            break;
                        }
                        if index_ >= last_index {
                            break;
                        }
                        index_ +=1;
                    }
                }
                {
                    let mut index_ = i - 1;
                    while true {
                        if nums[index_] < n {
                            behind_number = Some(nums[index_]);
                            break;
                        }
                        if nums[index_] > n  {
                            break;
                        }
                        if index_ <= 0 {
                            break;
                        }
                        index_ -=1;
                    }
                }

                //check if is hill
                if behind_number.is_some() && front_number.is_some() {
                    counter_result +=1;
                    continue;
                }
            }

            {
                //valley
                let mut behind_number: Option<i32> = None;
                let mut front_number: Option<i32> = None;
                {
                    let mut index_ = i +1;
                    while true {
                        if nums[index_] > n {
                            front_number = Some(nums[index_]);
                            break;
                        }
                        if nums[index_] < n  {
                            break;
                        }
                        if index_ >= last_index {
                            break;
                        }
                        index_ +=1;
                    }
                }
                {
                    let mut index_ = i - 1;
                    while true {
                        if nums[index_] > n {
                            behind_number = Some(nums[index_]);
                            break;
                        }
                        if nums[index_] < n  {
                            break;
                        }
                        if index_ <= 0 {
                            break;
                        }
                        index_ -=1;
                    }
                }

                //check if is valley
                if behind_number.is_some() && front_number.is_some() {
                    counter_result +=1;
                    continue;
                }
            }
        }
        println!("{:?}", counter_result);
        counter_result
    }
}

// //return de index and the
// pub fn check_next_hill_posible_number() -> (i32, i32) {}

// pub fn check_next_hill_posible_number() -> (i32, i32) {}
