fn main() {}

struct Solution;
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut chars = s
            .chars()
            .filter(|v| v.is_alphanumeric())
            .map(|v| v.to_ascii_lowercase());
        while let (Some(a), Some(b)) = (chars.next(), chars.next_back()) {
            if a != b {
                return false;
            }
        }
        true
    }
}
