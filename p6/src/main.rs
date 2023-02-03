struct Solution {}

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }

        let mut result = String::new();
        result.reserve_exact(s.len());

        let width = (num_rows - 1) * 2;
        for i in 0..num_rows {
            let gap = i * 2;
            let steps = [width - gap, gap];
            let mut step_index = 0;

            let mut u = i;
            while (u as usize) < s.len() {
                if steps[step_index] != 0 {
                    result.push(s.chars().nth(u as usize).unwrap());
                    u = u + steps[step_index];
                }

                step_index = (step_index + 1) % 2;
            }
        }

        return result;
    }
}

fn main() {
    println!(
        "Expect: 'PAHNAPLSIIGYIR'\nOutput: '{}'",
        Solution::convert("PAYPALISHIRING".to_string(), 3)
    );
    println!(
        "Expect: 'ABC'\nOutput: '{}'",
        Solution::convert("ABC".to_string(), 1)
    );
}
