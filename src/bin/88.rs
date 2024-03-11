fn main() {}

struct Solution;
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i = nums1.len();
        let (mut m, mut n) = (m as usize, n as usize);

        while n > 0 {
            if m > 0 && nums1[m - 1] > nums2[n - 1] {
                nums1[i - 1] = nums1[m - 1];
                m -= 1;
            } else {
                nums1[i - 1] = nums2[n - 1];
                n -= 1;
            }
            i -= 1;
        }
    }
}

//nums1 = [1,2,2,3,5, 6, 8], m = 3, nums2 = [2,5,6], n = 3
