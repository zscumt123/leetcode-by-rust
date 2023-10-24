use std::collections::HashMap;

fn main() {}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut cache: HashMap<i32, usize> = HashMap::new();
    for (i, val) in nums.iter().enumerate() {
        let diff = target - val;
        if let Some(index) = cache.get(&diff) {
            return vec![i as i32, (*index) as i32];
        } else {
            cache.insert(*val, i);
        }
    }
    unreachable!("a solution has exists")
}
