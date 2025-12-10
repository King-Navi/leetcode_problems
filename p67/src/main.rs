struct Solution;
fn main() {
    println!("Hello, world!");
}
impl Solution {
     pub fn add_binary(a: String, b: String) -> String {
        let a_bytes = a.as_bytes();
        let b_bytes = b.as_bytes();

        let mut i = a_bytes.len() as i32 - 1;
        let mut j = b_bytes.len() as i32 - 1;
        let mut carry = 0;
        let mut res: Vec<u8> = Vec::with_capacity(a_bytes.len().max(b_bytes.len()) + 1);

        while i >= 0 || j >= 0 || carry > 0 {
            let bit_a = if i >= 0 {
                (a_bytes[i as usize] - b'0') as i32
            } else {
                0
            };

            let bit_b = if j >= 0 {
                (b_bytes[j as usize] - b'0') as i32
            } else {
                0
            };

            let sum = bit_a + bit_b + carry;
            let bit = (sum & 1) as u8;   // sum % 2
            carry = (sum >> 1);          // sum / 2

            res.push(b'0' + bit);

            i -= 1;
            j -= 1;
        }

        res.reverse();
        String::from_utf8(res).unwrap()
    }
}