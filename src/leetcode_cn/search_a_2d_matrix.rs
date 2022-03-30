#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        // find the suitable search row
        let mut i = 0;
        for row in &matrix {
            if (row[0]..=row[row.len() - 1]).contains(&target) {
                break;
            }
            i+=1;
        }
        // no row suitable for target
        if i == matrix.len() {
            return false;
        }
        let row = &matrix[i];
        Self::binary_search(row, target, 0, row.len())
    }

    fn binary_search(v: &Vec<i32>, num: i32, mut lo: usize, mut hi: usize) -> bool {
        while lo < hi {
            let mid = (lo + hi) / 2;
            if v[mid] == num {
                return true;
            } else if v[mid] < num {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }

        false
    }
}

#[test]
fn test() {

}
