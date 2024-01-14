fn main() {}
struct Solution;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut cache = HashMap::new();

        for word in strs {
            let mut current = word.as_bytes().to_vec();
            current.sort();
            cache.entry(current).or_insert(vec![]).push(word);
        }
        cache.into_iter().map(|(_, v)| v).collect()
    }
}
