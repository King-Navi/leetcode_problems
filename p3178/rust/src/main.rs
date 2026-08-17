use std::print;

struct Solution;
// impl Solution {
//     pub fn number_of_child(n: i32, k: i32) -> i32 {
//         let max_num = n -1;
//         let mut diretion = true;
//         let mut child = 0;
//         let mut counter=0;
//         loop {
//             if diretion {
//                 child +=1;
//                 counter+=1;
//                 if child == max_num {
//                     diretion= !diretion;
//                 }
//             }else {
//                 child -=1;
//                 counter+=1;
//                 if child == 0 {
//                     diretion= !diretion;
//                 }
//             }
//             if counter == k {
//                 break;
//             }
//         }
//         return child;
//     }
// }
impl Solution {
    pub fn number_of_child(n: i32, k: i32) -> i32 {
        let dir = (n-1) % k;
        if dir % 2 ==0 {
            //izq a der
            println!("izq a der");
        }else {
            
            println!("der a izq");
        }
        0
    }
}
fn main() {
    println!("{}", Solution::number_of_child(3,5));
    println!("{}", Solution::number_of_child(5,6));
    println!("{}", Solution::number_of_child(4,2));
}
