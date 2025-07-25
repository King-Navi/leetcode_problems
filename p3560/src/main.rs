pub struct Solution;

fn main() {
    Solution::min_cutting_cost(49898,109372,62703);
}

impl Solution {
    pub fn min_cutting_cost(n: i32, m: i32, k: i32) -> i64 {
        let mut cost_n : i64 = 0;
        let mut cost_m : i64 = 0;

        if n <= k {
            cost_n += 0;
        }else {
            //calculate cost
            cost_n = cost(n, k);
        }

        if m <= k {
            cost_m += 0;
        }else {
            //calculate cost
            cost_m = cost(m, k);
        }
        cost_m + cost_n
    }
}

pub fn cost(log_size: i32, k : i32) -> i64{
    // a <= k
    // b <= k

    //x = a + b    Log_size
    // Log_size - a = b
    //y = a * b    Cost
    //y = a * (Log_size - a)
    let mut minor_cost : i64 = 0;
    for a in 1..k+1  {
        if a > k {
            break;
        }
        let b = log_size - a;
        if b <= k {
            let cost : i64 = a as i64 * b as i64;
            if cost  < minor_cost || minor_cost == 0 {
                minor_cost = cost;
            }
            println!("a : {a}");
            println!("b : {b}");
            println!("cost : {cost}");
        }
    }
    minor_cost
}