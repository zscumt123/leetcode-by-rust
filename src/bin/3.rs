use std::collections::HashMap;
fn main() {}

pub fn length_of_longest_substring(s: String) -> i32 {
    let (mut i, mut res) = (0, 0);
    let mut cache: HashMap<char, usize> = HashMap::new();
    let iter = s.chars().enumerate();
    for (index, val) in iter {
        if let Some(s) = cache.get(&val) {
            i = i.max(*s + 1)
        }

        cache.insert(val, index);
        res = res.max(index - i + 1)
    }
    res as i32
}
