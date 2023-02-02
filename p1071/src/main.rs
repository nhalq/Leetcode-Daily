struct Solution {}

impl Solution {
    pub fn can_divide(dividend: &String, divisor: &String) -> bool {
        if dividend.len() < divisor.len() {
            return false;
        }

        if dividend.len() % divisor.len() != 0 {
            return false;
        }

        for i in (0..dividend.len()).step_by(divisor.len()) {
            if &dividend[i..i + divisor.len()] != divisor {
                return false;
            }
        }

        return true;
    }

    pub fn gcd_of_strings(lhs: String, rhs: String) -> String {
        for gcd_size in (1..lhs.len().min(rhs.len()) + 1).rev() {
            let gcd = &lhs[..gcd_size].to_string();
            if Solution::can_divide(&lhs, gcd) && Solution::can_divide(&rhs, gcd) {
                return gcd.to_string();
            }
        }

        return "".to_string();
    }
}

fn main() {
    println!("Expect: ABC");
    println!(
        "Output: {}",
        Solution::gcd_of_strings("ABCABC".to_string(), "ABC".to_string())
    );
}
