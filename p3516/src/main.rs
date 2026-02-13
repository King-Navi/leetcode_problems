pub struct Solution;
fn main() {
    let s  =Solution::find_closest(2, 7, 4);
    println!("{s}");
}

impl Solution {
    pub fn find_closest(x: i32, y: i32, z: i32) -> i32 {
        let x = steps_requiered(&x, &z);
        let y = steps_requiered(&y, &z);
        if x == y {
            return 0;
        }else if x > y {
            return 2;
        }else {
            return 1;
        }
    }
    
}

pub fn steps_requiered(n : & i32 , z : & i32) -> i32{
    if n == z {
        return 0;
    }else if n > z {
        return n- z;
    }else {
        return z-n;
    }

}