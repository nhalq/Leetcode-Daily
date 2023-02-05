use std::collections::HashMap;

struct Solution {
    //
}

impl Solution {
    pub fn check_inclusion(perm: String, given: String) -> bool {
        let mut occurs = HashMap::new();
        for c in perm.chars() {
            let count = occurs.entry(c).or_insert(0);
            *count += 1;
        }

        for c in (&given[0..perm.len()]).chars() {
            let count = occurs.entry(c).or_insert(0);
            *count -= 1;
        }

        let mut matches = occurs.values().filter(|&&v| v != 0).count();
        if matches == 0 {
            return true;
        }

        for i in perm.len()..given.len() {
            let c = given.chars().nth(i).unwrap();
            let count = occurs.entry(c).or_insert(0);
            if count == &0 {
                matches += 1;
            }
            *count -= 1;
            if count == &0 {
                matches -= 1;
            }

            let c = given.chars().nth(i - perm.len()).unwrap();
            let count = occurs.entry(c).or_insert(0);
            if count == &0 {
                matches += 1;
            }
            *count += 1;
            if count == &0 {
                matches -= 1;
            }

            if matches == 0 {
                return true;
            }
        }

        return false;
    }
}

fn main() {
    let cases = [
        (
            Solution::check_inclusion("ab".to_string(), "eidbaooo".to_string()),
            true,
        ),
        (
            Solution::check_inclusion("ab".to_string(), "eidboaoo".to_string()),
            false,
        ),
    ];

    for (i, (output, expect)) in cases.iter().enumerate() {
        println!("==== Case #{} ", i + 1);
        println!("Output: {}\nExpect: {}", output, expect);
    }
}
