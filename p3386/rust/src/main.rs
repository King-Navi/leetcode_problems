
struct Solution;
impl Solution {
    pub fn button_with_longest_time(events: Vec<Vec<i32>>) -> i32 {
        let max =events.len();
        let (mut index_result, mut max_time) = (events[0][0],events[0][1]);
        println!("first : {} {}", index_result, max_time);
        
        for i in 1..max {
            let arr= &events[i];
            let index = arr[0];
            let time= arr[1];
            if let Some(last_vec) = events.get(i-1)  {
                let amount = time - last_vec[1];
                println!("-----------------");
                println!("amunt {} ", amount);
                println!("Max amount {} ", max_time);
                if max_time < amount {
                    index_result = index;                                     
                    max_time = amount;
                }
                if max_time == amount {
                    if index_result > index {
                        index_result = index;
                    }
                }
                println!("-----------------");
                println!("amunt {} ", amount);
                println!("index {} ", index);
                println!("time {} ", time);
                
            }else {
                if max_time <= time {
                    if index < index_result {
                        index_result = index;
                    } 
                    max_time = time;
                }
                println!("--------ELSE---------");
                println!("{} ", index);
                println!("{} ", time);
            }
        }
        println!("-----------------");
        index_result
    }
}

fn main() {
    //println!("{}", Solution::button_with_longest_time(vec![vec![1,2],vec![2,5],vec![3,9],vec![1,15]]));
    //println!("{}", Solution::button_with_longest_time(vec![vec![10,5],vec![1,7]]));
    println!("{}", Solution::button_with_longest_time(vec![vec![9,4],vec![19,5],vec![2,8],vec![3,11],vec![2,15]]));
    println!("{}", Solution::button_with_longest_time(vec![vec![5,4],vec![20,14]]));
}
