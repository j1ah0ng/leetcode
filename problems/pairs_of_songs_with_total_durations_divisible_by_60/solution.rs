use std::collections::HashMap;

impl Solution {
    pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        let mut cnt: i32 = 0;
        let mut set: HashMap<i32, i32> = HashMap::new();
        for t in time {
            let modulus = t % 60;
            let search = &(match 60 - modulus {
                60 => 0,
                n => n
            });
            if set.get(search).is_some() {
                cnt += set.get(search).expect("Something went wrong.");
            }
            match set.get(&modulus) {
                Some(n) => set.insert(modulus, n + 1),
                None => set.insert(modulus, 1)
            };
        }
        return cnt
    }
}