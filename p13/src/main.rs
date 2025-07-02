use std::{collections::{ HashMap}};

pub struct Solution; 

/*
I             1
V             5
X             10
L             50
C             100
D             500
M             1000
*/
fn symbol_value() -> HashMap<char, i32> {
    HashMap::from([
        ('I', 1), 
        ('V', 5), 
        ('X', 10),
        ('L', 50), 
        ('C', 100), 
        ('D', 500),
        ('M', 1000),
    ])
}

fn main() {
    let solution = Solution::roman_to_int(String::from("XII"));
    println!("El número es {}", solution);
}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut accumulation : i32 = 0;
        let max = s.chars().count();
        for (i, ch) in s.chars().enumerate() {
            if let Some(ref_ref) = symbol_value().get(&ch){
                let valor_actual = *ref_ref;
                if i + 1 == max{
                    accumulation += valor_actual
                }else {
                    if let Some(next_ch) = s.chars().nth(i + 1) {
                        if let Some(value) = symbol_value().get(&next_ch) {
                            let numero_siguiente = * value;
                            if numero_siguiente <= valor_actual {
                                accumulation += valor_actual;
                            }else if numero_siguiente > valor_actual{
                                accumulation -= valor_actual;
                            }
                        }
                    }
                }
            }
        }
        accumulation
    }
}