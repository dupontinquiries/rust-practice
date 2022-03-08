// was able to do this problem by myself (the first part, at least)
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

use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for a in 0..nums.len() {
            for b in 0..nums.len() {
                if a == b { continue; }
                if nums[a] + nums[b] == target { return vec![a as i32, b as i32]; }
            }
        }
        return vec![0,0];
    }

    // taken from https://github.com/aylei/leetcode-rust/blob/master/src/solution/s0001_two_sum.rs
    pub fn two_sum_efficient(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // make a hash map with same size as nums
        let mut map = HashMap::with_capacity(nums.len());
        // add elements to the hash map until two add up to the target
        for (index, num) in nums.iter().enumerate() {
            // check to see if map has the complement
            match map.get(&(target - num)) {
                None => {
                    // if no solution found, keep adding elements portion
                    map.insert(num, index);
                }
                // otherwise return the found pair
                Some(sub_index) => return vec![*sub_index as i32, index as i32],
            }
        }
        // return an empty vec if not found
        vec![]
    }
}

fn main() {
    println!("running...");
    // testing On^2 algo
    assert_eq!(Solution::two_sum(vec![2,7,11,15], 9), vec![0,1]);
    assert_eq!(Solution::two_sum(vec![3,2,4], 6), vec![1,2]);
    assert_eq!(Solution::two_sum(vec![3,3], 6), vec![0,1]);

    // testing more efficient algorithm
    assert_eq!(Solution::two_sum_efficient(vec![1,1,1,2,7,11,15,10,10], 9), vec![3,4]);
    assert_eq!(Solution::two_sum_efficient(vec![2,7,11,15], 9), vec![0,1]);
    assert_eq!(Solution::two_sum_efficient(vec![3,2,4], 6), vec![1,2]);
    assert_eq!(Solution::two_sum_efficient(vec![3,3], 6), vec![0,1]);
    println!("passed all tests!");
}
