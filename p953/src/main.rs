use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn is_greater(str_left: &String, str_right: &String, order: &HashMap<u8, usize>) -> bool {
        let bytes_lhs = str_left.as_bytes();
        let bytes_rhs = str_right.as_bytes();

        for i in 0..str_left.len() {
            if i == str_right.len() {
                return true;
            }

            let char_left = order.get(&bytes_lhs[i]).unwrap();
            let char_right = order.get(&bytes_rhs[i]).unwrap();
            if char_left == char_right {
                continue;
            }

            return char_left > char_right;
        }

        return false;
    }
    pub fn is_alien_sorted(words: Vec<String>, str_order: String) -> bool {
        let mut order = HashMap::new();
        str_order.chars().enumerate().for_each(|(i, c)| {
            order.insert(c as u8, i);
        });

        for pair in words.windows(2) {
            let (str_left, str_right) = (&pair[0], &pair[1]);
            if Solution::is_greater(str_left, str_right, &order) {
                return false;
            }
        }

        return true;
    }
}

fn main() {
    println!(
        "Output: {}\nResult: {}",
        Solution::is_alien_sorted(
            vec!["hello".to_string(), "leetcode".to_string()],
            "hlabcdefgijkmnopqrstuvwxyz".to_string()
        ),
        true
    );
}
