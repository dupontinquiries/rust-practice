// was able to do this problem by myself
// this is a big deal because I am still learning rust

/**

Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

You can return the answer in any order.

Example 1:

Input: nums = [2,7,11,15], target = 9
Output: [0,1]
Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].

Example 2:

Input: nums = [3,2,4], target = 6
Output: [1,2]

Example 3:

Input: nums = [3,3], target = 6
Output: [0,1]

Constraints:

2 <= nums.length <= 10^4
-10^9 <= nums[i] <= 10^9
-10^9 <= target <= 10^9
Only one valid answer exists.


Follow-up: Can you come up with an algorithm that is less than O(n2) time complexity?

*/

pub struct Solution {}

impl Solution {
    pub fn two_sum_greedy(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for a in 0..nums.len() {
            for b in 0..nums.len() {
                if a == b { continue; }
                if nums[a] + nums[b] == target { return vec![a as i32, b as i32]; }
            }
        }
        return vec![0,0];
    }
}

fn main() {
    println!("running...");
    // testing On^2 algo
    assert_eq!(Solution::two_sum_greedy(vec![2,7,11,15], 9), vec![0,1]);
    assert_eq!(Solution::two_sum_greedy(vec![3,2,4], 6), vec![1,2]);
    assert_eq!(Solution::two_sum_greedy(vec![3,3], 6), vec![0,1]);

    // testing more efficient algorithm
    // TODO: finish
    assert_eq!(Solution::two_sum(vec![2,7,11,15], 9), vec![0,1]);
    assert_eq!(Solution::two_sum(vec![3,2,4], 6), vec![1,2]);
    assert_eq!(Solution::two_sum(vec![3,3], 6), vec![0,1]);
    println!("passed all tests!");
}
