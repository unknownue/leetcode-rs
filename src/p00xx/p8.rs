//!
//! String to Integer (atoi)
//!
//! https://leetcode.com/problems/string-to-integer-atoi/
//!
//! Implement atoi which converts a string to an integer.
//!
//! The function first discards as many whitespace characters as necessary until the first non-whitespace character is found. Then, starting from this character, takes an optional initial plus or minus sign followed by as many numerical digits as possible, and interprets them as a numerical value.
//!
//! The string can contain additional characters after those that form the integral number, which are ignored and have no effect on the behavior of this function.
//!
//! If the first sequence of non-whitespace characters in str is not a valid integral number, or if no such sequence exists because either str is empty or it contains only whitespace characters, no conversion is performed.
//!
//! If no valid conversion could be performed, a zero value is returned.
//!
//! Note:
//!
//! Only the space character ' ' is considered as whitespace character.
//!
//! Assume we are dealing with an environment which could only store integers within the 32-bit signed integer range: [−2^31,  2^31 − 1].
//!
//! If the numerical value is out of the range of representable values, INT_MAX (2^31 − 1) or INT_MIN (−2^31) is returned.
//!
//! ## Example 1:
//! ```text
//! Input: "42"
//! Output: 42
//! ```
//!
//! ## Example 2:
//! ```text
//! Input: "   -42"
//! Output: -42
//! Explanation: The first non-whitespace character is '-', which is the minus sign.
//!          Then take as many numerical digits as possible, which gets 42.
//! ```
//!
//! ## Example 3:
//! ```text
//! Input: "4193 with words"
//! Output: 4193
//! Explanation: Conversion stops at digit '3' as the next character is not a numerical digit.
//! ```
//!
//! ## Example 4:
//! ```text
//! Input: "words and 987"
//! Output: 0
//! Explanation: The first non-whitespace character is 'w', which is not a numerical digit or a +/- sign. Therefore no valid conversion could be performed.
//! ```
//!
//! ## Example 5:
//! ```text
//! Input: "-91283472332"
//! Output: -2147483648
//! Explanation: The number "-91283472332" is out of the range of a 32-bit signed integer. Thefore INT_MIN(-2^31) is returned.
//! ```
//!


pub type Input  = String;
pub type Output = i32;

pub trait Solution {
    fn my_atoi(&self, str: String) -> i32;
}

// -----------------------------------------------------------------------------
/// Approach 0
pub struct Solution0;
impl Solution for Solution0 {

    fn my_atoi(&self, str: String) -> i32 {

        use std::i32;

        let mut chars = str.trim()
            .chars()
            .peekable();

        let first_char = chars.peek();
        let sign: i32 = match first_char {
            | Some('+') => {
                chars.next();
                1
            },
            | Some('-') => {
                chars.next();
                -1
            },
            | _ => 1,
        };

        let mut result: i32 = 0;
        const BOUNDARY1: i32 = i32::max_value() / 10;
        const BOUNDARY2: i32 = i32::max_value() % 10;

        while let Some(ch) = chars.next() {
            if let Some(digit) = ch.to_digit(10).map(|v| v as i32) {

                if result < BOUNDARY1 || (result == BOUNDARY1 && digit <= BOUNDARY2) {
                    result = result * 10 + digit;
                } else {
                    if sign == 1 {
                        return i32::max_value()
                    } else {
                        return i32::min_value()
                    }
                }
            } else {
                break
            }
        }

        result * sign
    }
}
// -----------------------------------------------------------------------------
