struct Solution;
impl Solution {
    pub fn check_two_chessboards(coordinate1: String, coordinate2: String) -> bool {
        let vec_coordinate_one = coordinate1.chars().collect::<Vec<char>>();            
        let vec_coordinate_two = coordinate2.chars().collect::<Vec<char>>();
        //1to row
        //2do colum
        check(&vec_coordinate_one[1], &vec_coordinate_one[0]) == check(&vec_coordinate_two[1], &vec_coordinate_two[0])
    }
}

fn check(row:&char,comlun:&char)-> bool{
    if invert(row) % 2 == 1 {
        //impar epmieza en blancas
        println!("impar epmieza en blancas");
        if to_number(comlun) % 2== 1 {
            //impar balncas
            true
        }else {
            //par nrgas
            false
        }
    }else {
        //par empieza en negras
        println!("impar epmieza en negras");
        if to_number(comlun) % 2== 1 {
            //impar negras
            false
        }else {
            //par blancas
            true
        }
    }
}

fn to_number(coor: &char) -> i32{
    match coor {
        'a'=> 1,
        'b'=> 2,
        'c'=> 3,
        'd'=> 4,
        'e'=> 5,
        'f'=> 6,
        'g'=> 7,
        'h'=> 8,
        _=> -1
    }
}
fn invert(coor: &char) -> i32{
    match coor {
        '8'=> 1,
        '7'=> 2,
        '6'=> 3,
        '5'=> 4,
        '4'=> 5,
        '3'=> 6,
        '2'=> 7,
        '1'=> 8,
        _=> -1
    }
}
fn main() {
    //a1 c3
    //a1 h3
    //println!("{:?}",Solution::check_two_chessboards("a1".to_string(), "c3".to_string()));
    Solution::check_two_chessboards("a1".to_string(), "h3".to_string());
    
}


/*
You are given two strings, coordinate1 and coordinate2, representing the coordinates of a square on an 8 x 8 chessboard.

Below is the chessboard for reference.



Return true if these two squares have the same color and false otherwise.

The coordinate will always represent a valid chessboard square. The coordinate will always have the letter first (indicating its column), and the number second (indicating its row).

 

Example 1:

Input: coordinate1 = "a1", coordinate2 = "c3"

Output: true

Explanation:

Both squares are black.

Example 2:

Input: coordinate1 = "a1", coordinate2 = "h3"

Output: false

Explanation:

Square "a1" is black and "h3" is white.
*/