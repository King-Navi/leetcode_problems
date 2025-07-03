pub  struct Solution;

fn main() {
    #[allow(unused_variables)]
    let vector_exam: Vec<String> = vec![String::from("")];
    #[allow(unused_variables)]
    let vector_exam2:Vec<String> = vec![String::from("sad")];
    #[allow(unused_variables)]
    let vector_exam3:  Vec<String>= vec!["va".to_string(); 2];
    let mut vector_exam4:  Vec<String> = vec![];
    //["caa","","a","acb"]
    vector_exam4.append(&mut vec!["caa".to_string()]);
    vector_exam4.append(&mut vec!["".to_string()]);
    vector_exam4.append(&mut vec!["a".to_string()]);
    vector_exam4.append(&mut vec!["acb".to_string()]);
    let sss = Solution::longest_common_prefix(vector_exam3);
    println!("{}",sss);
}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut result:String = String::new();
        let max_elements: usize = strs.len();
        if max_elements == 1 {
            //or 'return strs.into_iter().next().unwrap();'
            return strs[0].clone();
        }
        let idx_lrg = index_of_largest(&strs);
        if let Some (largest_word)  = strs.get(idx_lrg){
            let mut my_strs = strs.clone();
            my_strs.remove(idx_lrg);
            
            let mut new_index :usize = 0;
            for c in largest_word.chars()  {
                let actual_vector = return_vector_char(&my_strs, new_index);
                if compare_all_chars(&c, &actual_vector) {
                    result.push(c);
                }else {
                    break;
                }
                new_index +=1;
            }
        };
        
        result
    }

}

pub fn index_of_largest(strs: &Vec<String>) -> usize{
        let mut index_of_largest: usize = 0;
        let mut index : usize = 0;
        for word in strs {
            let letters =word.chars().count();
            if letters >= index_of_largest {
                index_of_largest = index;
            }
            index +=1;
        }
        index_of_largest
    }

pub fn return_vector_char(strs : &Vec<String>, indexOfChar: usize)-> Vec<char>{
    let mut result :Vec<char> = vec![];
    for word in strs  {
        if let Some(c) = word.chars().nth(indexOfChar)   {
            result.push(c);
        }else {
            result.clear();
            break;
        }
    }
    result
}

pub fn compare_all_chars(charToCompare: &char, vchar:&Vec<char>)-> bool{
    if vchar.is_empty() {
        return false;
    }
    for c in vchar  {
        if charToCompare != c {
            return false;
        }
    }
    
    true
}