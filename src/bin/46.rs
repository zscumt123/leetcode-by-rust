fn main() {
    let res = vec![1, 2, 3];
    let a = Solution::permute(res);
    println!("a:{:?}", a);
}

struct Solution;
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let len = nums.len();
        let mut visit_cache = vec![false; len];
        println!("v:{:?}", visit_cache.len());
        fn dfs(
            current: &mut Vec<i32>,
            nums: &Vec<i32>,
            len: usize,
            collections: &mut Vec<Vec<i32>>,
            visit_cache: &mut Vec<bool>,
        ) {
            if current.len() == len {
                collections.push(current.clone());
                return;
            }

            for i in 0..len {
                if !visit_cache[i] {
                    current.push(nums[i]);
                    visit_cache[i] = true;
                    dfs(current, nums, len, collections, visit_cache);
                    current.pop();
                    visit_cache[i] = false;
                }
            }
        }
        dfs(&mut vec![], &nums, len, &mut result, &mut visit_cache);

        result
    }
}
