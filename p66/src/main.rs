struct Solution;
fn main() {
    println!("Hello, world!");
}

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut carry = 1; // empezamos sumando 1

        // Recorremos de derecha a izquierda
        for i in (0..digits.len()).rev() {
            let sum = digits[i] + carry;
            digits[i] = sum % 10;
            carry = sum / 10;

            // Si ya no hay acarreo, podemos parar
            if carry == 0 {
                break;
            }
        }

        // Si al final todavía hay acarreo, va un 1 al principio
        if carry > 0 {
            digits.insert(0, carry);
        }

        digits
    }
}