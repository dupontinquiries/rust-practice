
/**

https://leetcode.com/problems/palindrome-number/

Given an integer x, return true if x is palindrome integer.

An integer is a palindrome when it reads the same backward as forward.

For example, 121 is a palindrome while 123 is not.

Example 1:

Input: x = 121
Output: true
Explanation: 121 reads as 121 from left to right and from right to left.

Example 2:

Input: x = -121
Output: false
Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.

Example 3:

Input: x = 10
Output: false
Explanation: Reads 01 from right to left. Therefore it is not a palindrome.


Constraints:

-231 <= x <= 231 - 1

*/

pub struct Solution {}

impl Solution {
    /**
    Runtime: 15 ms, faster than 50.26% of Rust online submissions for Palindrome Number.
    Memory Usage: 2 MB, less than 93.44% of Rust online submissions for Palindrome Number.
    */
    pub fn is_palindrome_string(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        if x == 0 {
            return true;
        }
        let str = x.to_string();
        for i in 0..(str.chars().count()) {

            // println!("comparing {}:{}, {}:{}", i, str.chars().nth(i).unwrap(), str.len() - i, str.chars().nth(str.len() - i - 1).unwrap());

            if str.chars().nth(i) != str.chars().nth(str.chars().count() - i - 1) {
                return false;
            }
        }
        return true;
    }

    // fn is_palindrome_num_helper(n: i32) -> bool {
    //     return
    // }

    /**
    Runtime: 8 ms, faster than 80.29% of Rust online submissions for Palindrome Number.
    Memory Usage: 2.1 MB, less than 32.94% of Rust online submissions for Palindrome Number.
    */

    fn is_palindrome_num(n: i32) -> bool {
        // do not convert to string
        if n < 0 {
            return false;
        }
        if n == 0 {
            return true;
        }
        let mut n2 = n as i64;
        let mut factor = 0u32;
        while n2 > 0 {
            factor += 1;
            n2 /= 10;
        }
            // println!("len of num is {}", factor);
        n2 = n as i64;
        for i in 0..factor / 2 {
            // build first digit
            let mut first = n2 as i64;
            // divide until first is lsd
            first /= 10_i64.pow(i);
            // subtract rest
            first -= first / 10 * 10;
            // let first = (n2 / 10_i32.pow(i)) - ((n2 / 10_i32.pow(i)) / 10_i32.pow(i + 1) * 10);//_i32.pow(i));
            // build last digit
            let mut last = n2 as i64;
            // get last digit by dividing by ten factor - i - 1 times
            last /= 10_i64.pow(factor - i - 1);
            // then subtract the rest of the digits
            last -= n2 / 10_i64.pow(factor - i - 0) * 10;
            // let last = n2 - (n2 / 10_i32.pow(factor - i - 1));
                // println!("comparing {}: {}, {}", n2, last, first);
            if first != last {
                return false;
            }
            // n2 /= 10;
        }
        return true;
    }


    // fn is_palindrome_num(n: i32) -> bool {
    //     // do not convert to string
    //     if n < 0 {
    //         return false;
    //     }
    //     if n == 0 {
    //         return true;
    //     }
    //     let mut n2 = n;
    //     while n2 > 0 {
    //         let first = n2 - (n2 / 10 * 10);
    //         let mut last = n2;
    //         let mut factor = 0u32;
    //         print!("last: ");
    //         while last / 10 > 0 {
    //             last /= 10;
    //             factor += 1;
    //             print!(" {}", last);
    //         }
    //         print!("\n");
    //         println!("{} comparing {}, {}", n2, first, last);
    //         if last != first {
    //             return false;
    //         }
    //         println!("subtracting {} from {}", last * 10_i32.checked_pow(factor).unwrap(), n2);
    //         n2 -= last * 10_i32.checked_pow(factor).unwrap(); // remove msd
    //         n2 /= 10; // remove lsd
    //     }
    //     return true;
    // }
}

fn main() {
    println!("running...");

    assert_eq!(Solution::is_palindrome_string(121), true);
    assert_eq!(Solution::is_palindrome_string(-121), false);
    assert_eq!(Solution::is_palindrome_string(10), false);

    assert_eq!(Solution::is_palindrome_num(121), true);
    assert_eq!(Solution::is_palindrome_num(-121), false);
    assert_eq!(Solution::is_palindrome_num(10), false);
    println!("last test");
    assert_eq!(Solution::is_palindrome_num(1000021), false);
    println!("last test 2");
    assert_eq!(Solution::is_palindrome_num(1200021), true);
    println!("last test 3");
    assert_eq!(Solution::is_palindrome_num(1410110141), true);
    println!("passed all tests");
}
