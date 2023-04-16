// zigzag array problem from https://www.geeksforgeeks.org/convert-array-into-zig-zag-fashion/#main

pub trait ZigZag<T> {
    fn sorted_copy(arr: Vec<T>) -> Vec<T> {
        let mut arr = arr.clone();
        arr.sort();
        let mut result = vec![];
        let n = arr.len();
        for i in 0..n/2 {
            result.push(arr[n-i-1]);
            result.push(arr[i]);
        }
        if n%2 == 1 {
            result.push(arr[n/2]);
        }
        result
    }

    fn sort(arr: &mut Vec<T>);
}

impl<T: PartialOrd + Copy> ZigZag<T> {
    fn sort(arr: &mut Vec<T>) {
        let n = arr.len();
        for i in 0..n/2 {
            if arr[2*i] > arr[2*i+1] {
                arr.swap(2*i, 2*i+1);
            }
            if arr[2*i+1] < arr[2*i+2] {
                arr.swap(2*i+1, 2*i+2);
            }
        }
    }
}

// Given an array of distinct elements of size N, the task is to rearrange the elements of the array in a zig-zag fashion so that the converted array should be in the below form: arr[0] < arr[1]  > arr[2] < arr[3] > arr[4] < . . . . arr[n-2] < arr[n-1] > arr[n].

#[cfg(test)]
mod tests {
    #[test]
    fn example_1() {
        let arr = vec![4,3,7,8,6,2,1];
        let res = vec![3,7,4,8,6,2,1];
        ZigZag::<i32>::sort(&mut arr);
        assert_eq!(arr, res);
    }

    #[test]
    fn example_2() {
        let arr = vec![1,4,3,2];
        let res = vec![1,4,2,3];
        ZigZag::<i32>::sort(&mut arr);
        assert_eq!(arr, res);
    }
}
