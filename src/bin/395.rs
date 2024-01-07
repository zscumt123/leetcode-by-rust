fn main() {
    let s = Solution::longest_substring("aaabb".to_string(), 3);
    println!("result,{}", s)
}

use std::collections::hash_map::Entry;
use std::collections::HashMap;
struct Solution;
impl Solution {
    pub fn longest_substring(s: String, k: i32) -> i32 {
        let len = s.len();
        if len < k as usize {
            return 0;
        }
        let mut cache = HashMap::with_capacity(26);

        for char in s.chars() {
            let entry = cache.entry(char);
            match entry {
                Entry::Occupied(mut o) => {
                    *o.get_mut() += 1;
                }
                Entry::Vacant(v) => {
                    v.insert(1);
                }
            }
        }

        for (c, i) in cache.iter() {
            if *i < k {
                let mut res = 0;
                let child = s.split(*c).into_iter();
                for st in child {
                    res = res.max(Solution::longest_substring(st.to_string(), k));
                }
                return res;
            }
        }

        return s.len() as i32;
    }
}
