pub struct Solution {}

impl SolutionVec {

    pub fn merge(left: &Vec<i32>, right: &Vec<i32>) -> Vec<i32> {
        let mut i = 0;
        let mut j = 0;
        let mut merged: Vec<i32> = Vec::new();

        while i < left.len() && j < right.len() {
            if left[i] < right[j] {
                merged.push(left[i]);
                i = i + 1;
            } else {
                merged.push(right[j]);
                j = j + 1;
            }
        }

        if i < left.len() {
            while i < left.len() {
                merged.push(left[i]);
                i = i + 1;
            }
        }

        if j < right.len() {
            while j < right.len() {
                merged.push(right[j]);
                j = j + 1;
            }
        }

        merged
    }

    pub fn merge_sort(v: &Vec<i32>) -> &Vec<i32> {
        if vec.len() < 2 {
            // return the list if it is already sorted
            vec.to_vec()
        } else {
            // split the list in half and send to recursive call
            let size = vec.len() / 2;
            let left = merge_sort(&vec[0..size].to_vec());
            let right = merge_sort(&vec[size..].to_vec());
            let merged = merge(&left, &right);
            merged
        }
    }
}

fn main() {
    println!("Running tests...");
    assert_eq!(SolutionVec::merge_sort(vec![1]), vec![1]);
    assert_eq!(SolutionVec::merge_sort(vec![3, 2, 1]), vec![1, 2, 3]);
    println!("finished all tests");
}
