struct Solution;
fn main() {
    Solution::convert("PAYPALISHIRING".to_string(), 3);
    Solution::convert("PAYPALISHIRING".to_string(), 4);
    Solution::convert("A".to_string(), 4);
}

//first  row  (initial letter)   3  ( + 2 )

//last row  (initial letter numROw -1)  next letter numRow (- 1)
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 || num_rows as usize >= s.len() {
            return s;
        }
        
        let mut result = String::new();
        let iterations = num_rows as usize + 1;
        let chars: Vec<char> = s.chars().collect();
        let max_number_chars = chars.len();
        let mut incremenet_2 = (num_rows as usize - 2) * 2;
        let mut increment_3 = 2;
        for i in 1..iterations {
            if i == 1 {
                {
                    let mut index = 0;
                    let increment = 2 * (num_rows - 3) as usize + 4;
                    while index < max_number_chars {
                        result.push(chars[index]);
                        index += increment;
                    }
                }
                continue;
            }
            if i == num_rows as usize {
                {
                    let mut index = num_rows as usize - 1;
                    let increment = 2 * (num_rows - 3) as usize + 4;
                    while index < max_number_chars {
                        result.push(chars[index]);
                        index += increment;
                    }
                }
                break;
            }
            {
                let mut index = 0;
                let jump_letters = i - 1;                index += jump_letters;
                if index >= max_number_chars {
                    break;
                }
                result.push(chars[index]);
                {    
                    while index < max_number_chars {
                        index += incremenet_2;
                        if index >= max_number_chars {
                            break;
                        }
                        result.push(chars[index]);
                        index += increment_3;
                        if index >= max_number_chars {
                            break;
                        }
                        result.push(chars[index]);
                    }
                    increment_3 += 2;
                    incremenet_2 -= 2;
                }
            }
        }
        println!("{:?}", result);
        result
    }
}


//row 4
/*

P     I    N           // 0  , 6 , 12
A   L S  I G           // 1,5,  7,11,  13
Y A   H R              // 2,4,  8,10,  14,16 
P     I                // 3  , 9

*/

//row 5

/*

P        I     N     // 0 , 8, 16
A      L S   I G     // 1,7,    9,15,  17
Y    A   H  R        // 2,6,    10,14
P  3     I Q         // 3,5,    11,13
R	     2           // 4 , 12

*/


//ROW 6
/*

P        I      N
A      L S    I G
Y    A   H   R
P  3     I  Q
R E	 2 E
E	 E

Fila 0:   0       10          20   → P         I         N
Fila 1:   1     9 11       19 21   → A       L S       I 
Fila 2:   2   8   12     18        → Y     A   H     R
Fila 3:   3  7    13   17          → P   3     I   Q
Fila 4:   4 6     14 16            → R E       2 Q
Fila 5:   5       15               → E         E


*/