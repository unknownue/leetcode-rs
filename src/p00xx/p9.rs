//!
//! Palindrome Number
//!
//! https://leetcode.com/problems/palindrome-number/
//!
//! Determine whether an integer is a palindrome.
//!
//! An integer is a palindrome when it reads the same backward as forward.
//! 
//! **Example 1:**
//! ```text
//! Input: 121
//! Output: tru
//! ```
//! 
//! **Example 2:**
//! ```text
//! Input: -121
//! Output: false
//! Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
//! ```
//! 
//! **Example 3:**
//! ```text
//! Input: 10
//! Output: false
//! Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
//! ```
//!


pub type Input  = i32;
pub type Output = bool;

pub trait Solution {
    fn is_palindrome(&self, x: i32) -> bool;
}

// -----------------------------------------------------------------------------
/// Approach 0: Convert the number into digits.
pub struct Solution0;
impl Solution for Solution0 {

    fn is_palindrome(&self, x: i32) -> bool {
        
        if x < 0 { return false }
        if x == 0 { return true }

        let mut nums = Vec::new();
        let mut x = x;

        while x > 0 {
            nums.push(x % 10);
            x /= 10;
        }

        let mut i = 0;
        let mut j = nums.len() - 1;

        while i < j {
            if nums[i] != nums[j] {
                return false
            }
            i += 1;
            j -= 1;
        }

        true
    }
}
// -----------------------------------------------------------------------------


// -----------------------------------------------------------------------------
/// Approach 1: Convert the number into string.
pub struct Solution1;
impl Solution for Solution1 {

    fn is_palindrome(&self, x: i32) -> bool {
        
        if x < 0 { return false }
        
        let x: String = x.to_string();
        let x_rev: String = x.chars().rev().collect();

        x == x_rev
    }
}
// -----------------------------------------------------------------------------

// -----------------------------------------------------------------------------
/// Approach 2: Revert half of the number.
pub struct Solution2;
impl Solution for Solution2 {

    fn is_palindrome(&self, x: i32) -> bool {
        
        if x < 0 || (x % 10 == 0 && x != 0) { return false }
        
        let mut x   = x;
        let mut rev = 0;

        while x > rev {
            rev = rev * 10 + x % 10;
            x /= 10;
        }
        
        x == rev || x == rev / 10
    }
}
// -----------------------------------------------------------------------------
