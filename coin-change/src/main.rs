
pub struct Solution {}

impl Solution {
    pub fn coin_change(coins: Vec<u64>, amount: u64) -> u64 {
        let n: usize = coins.len();
        let mut t: Vec<u64> = vec![0; (amount+1) as usize];
        t[0] = 1;
        for i in 0..n {
            for j in 1..(amount+1) as usize {
                if j as u64 >= coins[i] {
                    t[j] += t[j - coins[i] as usize];
                }
            }
        }
        t[amount as usize]
    }
}

fn main() {
    println!("running tests...");
    assert_eq!(7, Solution::coin_change(vec![1,2,3], 6));
    assert_eq!(3, Solution::coin_change(vec![1,2], 4));
    assert_eq!(37191, Solution::coin_change(vec![1,2,3,6,9], 120));
    assert_eq!(1, Solution::coin_change(vec![1], 6));
    println!("passed all tests");
}
