struct Solution;


impl Solution {
    pub fn number_of_alternating_groups(colors: Vec<i32>) -> i32 {
        let len =colors.len(); 
        if  len<= 3  {
            return 0;
        }
        let mut left_idx = 0;
        let mut middle_idx = 1;
        let mut rigth_idx = 2;
        let mut resulr = 0;
        let mut evaluation = 0;
        while evaluation < len  {
            if (colors[left_idx % len ] == colors[rigth_idx % len])
              && colors[middle_idx % len] != colors[left_idx % len]
              && colors[middle_idx % len] != colors[rigth_idx % len]{
                resulr +=1;    
            }
            rigth_idx +=1;
            middle_idx +=1;
            left_idx +=1;
            evaluation +=1;
        }
        print!("{}",evaluation);
        resulr
    }
}

fn main(){
    
}