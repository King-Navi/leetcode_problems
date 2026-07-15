use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn winning_player_count(n: i32, pick: Vec<Vec<i32>>) -> i32 {
        let mut hashmap : HashMap<i32, HashMap<i32,i32>> = HashMap::new();
        let mut result = 0_i32;
        for v in pick {
            if let Some(score)= hashmap.get_mut(&v[0])  {
                *score.entry(v[1]).or_insert(0) +=1;
                
            }else {
                let mut nuevo: HashMap<i32, i32> = HashMap::new();
                nuevo.insert(v[1], 1);
                hashmap.insert(v[0], nuevo);
            }
        }
        'main: for (id_player, scores ) in hashmap {
            for (color, quantity) in scores {
                if id_player < quantity {
                    result += 1;
                    continue 'main;
                }
            }
        }

        result
    }
}
fn main() {
    println!("Hello, world!");
}
