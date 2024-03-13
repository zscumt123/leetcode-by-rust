fn main() {}
struct Solution;
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let len1 = matrix.len();
        let len2 = matrix[0].len();
        let mut row = vec![1; len1];
        let mut col = vec![1; len2];
        for i in 0..len1 {
            for j in 0..len2 {
                if matrix[i][j] == 0 {
                    row[i] = 0;
                    col[j] = 0;
                }
            }
        }

        for i in 0..len1 {
            for j in 0..len2 {
                if row[i] == 0 || col[j] == 0 {
                    matrix[i][j] = 0;
                }
            }
        }
    }
}
