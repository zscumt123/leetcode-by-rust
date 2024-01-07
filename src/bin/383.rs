fn main() {
    let res = can_construct1("aa".to_string(), "aab".to_string());
    println!("aa,{}", res)
}

/*
给你两个字符串：ransomNote 和 magazine ，判断 ransomNote 能不能由 magazine 里面的字符构成。

如果可以，返回 true ；否则返回 false 。

magazine 中的每个字符只能在 ransomNote 中使用一次。



示例 1：

输入：ransomNote = "a", magazine = "b"
输出：false
示例 2：

输入：ransomNote = "aa", magazine = "ab"
输出：false
示例 3：

输入：ransomNote = "aa", magazine = "aab"
输出：true
*/

use std::collections::hash_map::Entry;
use std::collections::HashMap;
pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut cache = HashMap::with_capacity(26);

    for char in magazine.chars() {
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

    for char in ransom_note.chars() {
        let entry = cache.entry(char);
        match entry {
            Entry::Occupied(mut o) => {
                let current = o.get_mut();
                if *current == 0 {
                    return false;
                }
                *current -= 1;
            }
            Entry::Vacant(_) => {
                return false;
            }
        }
    }
    return true;
}

pub fn can_construct1(ransom_note: String, magazine: String) -> bool {
    let mut cache = [0; 26];
    let a = b'a';
    for byte in magazine.bytes() {
        cache[(byte - a) as usize] += 1;
    }

    for b in ransom_note.bytes() {
        let index = (b - a) as usize;
        if cache[index] == 0 {
            return false;
        }
        cache[index] -= 1;
    }

    true
}
