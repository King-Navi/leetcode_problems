struct Solution;
impl Solution {
    pub fn max_height_of_triangle(red: i32, blue: i32) -> i32 {
        let mut result_red_start = 0;
        let mut result_blue_start = 0;
        {

            let mut red_start: bool = true;
            let mut red = red;
            let mut blue = blue;
            let mut next_level = 1;
            

            while true {
                if red_start && (red - next_level) >= 0 {
                    red -= next_level;
                    println!("RED RESTANTES {}", red);
                } else if !red_start && (blue - next_level) >= 0 {
                    blue -= next_level;
                    println!("BLUE RESTANTES {}", blue);
                } else {
                    break;
                }

                red_start = !red_start;
                result_red_start += 1;
                println!("{}", result_red_start);
                next_level += 1;
            }
        }

        {

            let mut red_start: bool = false;
            let mut red = red;
            let mut blue = blue;
            let mut next_level = 1;
            

            while true {
                if red_start && (red - next_level) >= 0 {
                    red -= next_level;
                    println!("RED RESTANTES {}", red);
                } else if !red_start && (blue - next_level) >= 0 {
                    blue -= next_level;
                    println!("BLUE RESTANTES {}", blue);
                } else {
                    break;
                }

                red_start = !red_start;
                result_blue_start += 1;
                println!("{}", result_blue_start);
                next_level += 1;
            }
        }

        if result_red_start > result_blue_start {
            result_red_start
        }else {
            result_blue_start
        }
    }
}

fn main() {
    println!("{}", Solution::max_height_of_triangle(10, 1));
}
