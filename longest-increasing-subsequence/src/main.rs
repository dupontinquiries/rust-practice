
pub struct Solution {}

// TODO: make polymorphic version of solution
impl Solution {
    pub fn longest_increasing_subsequence(v: Vec<u64>) -> u64 {
        if v.len() < 2 {
            return v.len() as u64;
        }
        let mut subproblems: Vec<u64> = vec![1;v.len()]; // don't need the +1
        for i in 2..v.len()+1 {
            let ii = v.len()-i;
            if v[ii] <= v[ii+1] {
                subproblems[ii] += subproblems[ii+1];
            }
        }
        // todo: use a pointer or iterator in this loop
        let mut highest: u64 = 0;
        for i in 0..subproblems.len() {
            if highest < subproblems[i] {
                highest = subproblems[i];
            }
        }
        highest
    } 
}

fn main() {
    println!("running tests...");
    assert_eq!(0, Solution::longest_increasing_subsequence(vec![]));
    assert_eq!(1, Solution::longest_increasing_subsequence(vec![1]));
    assert_eq!(2, Solution::longest_increasing_subsequence(vec![5,2,3,1]));
    assert_eq!(3, Solution::longest_increasing_subsequence(vec![9,0,2,3,1]));
    assert_eq!(3, Solution::longest_increasing_subsequence(vec![3,1,2,3,1]));
    assert_eq!(3, Solution::longest_increasing_subsequence(vec![1,2,3,1]));
    assert_eq!(7, Solution::longest_increasing_subsequence(vec![1,2,3,1,5,8,8,9,10,11,3,4,5,8,1]));
    println!("finished all tests");
}
