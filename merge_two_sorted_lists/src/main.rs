// this example is from https://github.com/aylei/leetcode-rust
// I am using it as a springboard to learn rust

/**
 * [21] Merge Two Sorted Lists
 *
 * Merge two sorted linked lists and return it as a new list. The new list should be made by splicing together the nodes of the first two lists.
 *
 * Example:
 *
 * Input: 1->2->4, 1->3->4
 * Output: 1->1->2->3->4->4
 *
 *
 */

// definition for singly-linked list (provided by leetcode)
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

// provided by leetcode
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// problem: https://leetcode.com/problems/merge-two-sorted-lists/
// discuss: https://leetcode.com/problems/merge-two-sorted-lists/discuss/?currentPage=1&orderBy=most_votes&query=

pub struct Solution {}
use crate::util::linked_list::{to_list, ListNode};

// submission codes start here

// recursive function
impl Solution {
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // create a base pointer that we can use as a starting point for our algorithm
        let mut dummy_head = Some(Box::new(ListNode{val:0,next:None}));
        // make a pointer that points to this head
        let mut head = &mut dummy_head;
        // make mut of l1_0 and l2_0
        let (mut l1, mut l2) = (l1, l2);
        // continue while there are any elements left
        while l1.is_some() || l2.is_some() {
            if l1.is_none() {
                // finish 2 if 1 is empty
                head.as_mut().unwrap().next = l2;
                break;
            } else if l2.is_none() {
                // finish 1 if 2 is empty
                head.as_mut().unwrap().next = l1;
                break;
            }
            // otherwise, unwrap the next two values and compare them to see which is smaller
            let next = if l1.as_ref().unwrap().val < l2.as_ref().unwrap().val {
                let (origin, next) = Solution::take_head(l1);
                l1 = origin;
                next
            } else {
                let (origin, next) = Solution::take_head(l2);
                l2 = origin;
                next
            };
            head.as_mut().unwrap().next = next;
            head = &mut head.as_mut().unwrap().next;
        }
        dummy_head.unwrap().next
    }

    // forces code to be inserted inline (at fn call) in most cases
    // - always: including across crate bounds
    // - can improve performance
    #[inline(always)]
    fn take_head(mut l: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        // unwrap next element
        // - we are going to move l so we must take it's next while we have a reference to it
        let l_next = l.as_mut().unwrap().next.take();
        // move value of l to next
        let next = l.take();
        // set l to unwrapped element
        l = l_next;
        // return the next element
        (l, next)
    }
}

// in our main function we assert the test cases
fn main() {
    // test 1
    // list1: 1, 2, 4
    // list2: 1, 3, 4
    // output should be 1, 1, 2, 3, 4, 4
    assert_eq!(
        Solution::merge_two_lists(to_list(vec![1, 2, 4]), to_list(vec![1, 3, 4])),
        to_list(vec![1, 1, 2, 3, 4, 4])
    );
}
