struct Solution;
fn main() {
    let s  =Solution::zigzag_traversal(vec![vec![1,2],vec![3,4]]);
    println!("{:?}", s);
    let s  =Solution::zigzag_traversal(vec![vec![2,1],vec![2,1],vec![2,1]]);
    println!("{:?}", s);
    let s  =Solution::zigzag_traversal(vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]]);
    println!("{:?}", s);
    let s  =Solution::zigzag_traversal(vec![vec![1,2,1,3],vec![5,15,7,3],vec![10,4,14,12]]);
    println!("{:?}", s);
}
impl Solution {
    pub fn zigzag_traversal(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result: Vec<i32> =vec![];
        for (i, ve) in grid.iter().enumerate() {
            if i % 2 == 0 || i == 0 {
                //Even
                for (j, &n) in ve.iter().enumerate() {
                    if j != 0 && j % 2 != 0 {
                        continue;
                    }
                    result.push(n);
                }
            }

            if i % 2 != 0 && i != 0 {
                //odd
                let mut vec_temp : Vec<i32>= vec![];
                for (j, &n) in ve.iter().enumerate() {
                    if j % 2 == 0 && i != 0 {
                        continue;
                    }
                    vec_temp.push(n);
                }
                vec_temp.reverse();
                result.append(&mut vec_temp);
            }
            

        }

        result
    }
}