//!
//! Remove Element
//!
//! https://leetcode.com/problems/remove-element/
//!
//! Given an array nums and a value val, remove all instances of that value [in-place](https://en.wikipedia.org/wiki/In-place_algorithm) and return the new length.
//!
//! Do not allocate extra space for another array, you must do this by **modifying the input array** [in-place](https://en.wikipedia.org/wiki/In-place_algorithm) with O(1) extra memory.
//!
//! The order of elements can be changed.
//!
//! It doesn't matter what you leave beyond the new length.
//!
//! **Example 1:**
//! ```text
//! Given nums = [3,2,2,3], val = 3,
//!
//! Your function should return length = 2, with the first two elements of nums being 2.
//!
//! It doesn't matter what you leave beyond the returned length.
//! ```
//!
//! **Example 2:**
//! ```text
//! Given nums = [0,1,2,2,3,0,4,2], val = 2,
//!
//! Your function should return length = 5, with the first five elements of nums containing 0, 1, 3, 0, and 4.
//!
//! Note that the order of those five elements can be arbitrary.
//! It doesn't matter what values are set beyond the returned length.
//! ```
//!
//! **Clarification**:
//!
//! Confused why the returned value is an integer but your answer is an array?
//!
//! Note that the input array is passed in by **reference**, which means modification to the input array will be known to the caller as well.
//! Internally you can think of this:
//!
//! ```text
//! // nums is passed in by reference. (i.e., without making a copy)
//! int len = removeElement(nums, val);
//!
//! // any modification to nums in your function would be known by the caller.
//! // using the length returned by your function, it prints the first len elements.
//! for (int i = 0; i < len; i++) {
//!    print(nums[i]);
//! }
//! ```
//!


#[derive(Debug, Clone)]
pub struct Input {
    pub nums: Vec<i32>,
    pub val: i32,
}
pub type Output = i32;

pub trait Solution {
    fn remove_element(&self, nums: &mut Vec<i32>, val: i32) -> i32;
}

// -----------------------------------------------------------------------------
/// Approach 0: Two Pointers.
pub struct Solution0;
impl Solution for Solution0 {

    fn remove_element(&self, nums: &mut Vec<i32>, val: i32) -> i32 {
        
        let mut index = 0;

        for i in 0..nums.len() {
            if nums[i] != val {
                nums[index] = nums[i];
                index += 1;
            }
        }

        nums.truncate(index);
        nums.len() as i32
    }
}
// -----------------------------------------------------------------------------


// -----------------------------------------------------------------------------
/// Approach 1: Swap Remove.
pub struct Solution1;
impl Solution for Solution1 {

    fn remove_element(&self, nums: &mut Vec<i32>, val: i32) -> i32 {
        
        let mut i = 0;
        while i < nums.len() {
            if nums[i] == val {
                nums.swap_remove(i);
            } else {
                i += 1;
            }
        }

        nums.len() as i32
    }
}
// -----------------------------------------------------------------------------

