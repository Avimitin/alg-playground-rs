#[allow(dead_code)]
struct RotateImage();
impl RotateImage {
    #[allow(dead_code)]
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for i in 0..n / 2 {
            for j in 0..n {
                Self::swap(matrix, i, j, n - i - 1, j);
            }
        }

        for i in 0..n {
            for j in 0..i {
                Self::swap(matrix, i, j, j, i);
            }
        }
    }

    fn swap(matrix: &mut Vec<Vec<i32>>, x1: usize, y1: usize, x2: usize, y2: usize) {
        let temp = matrix[x1][y1];
        matrix[x1][y1] = matrix[x2][y2];
        matrix[x2][y2] = temp;
    }
}

#[test]
fn test_rotate_image() {
    let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    RotateImage::rotate(&mut matrix);

    assert_eq!(matrix, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);
}
