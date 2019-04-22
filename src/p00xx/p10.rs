//!
//! Regular Expression Matching
//!
//! https://leetcode.com/problems/regular-expression-matching/
//!
//! Given an input string (s) and a pattern (p), implement regular expression matching with support for '.' and '*'.
//! ```ignore
//! '.' Matches any single character.
//! '*' Matches zero or more of the preceding element.
//! ```
//! The matching should cover the entire input string (not partial).
//! 
//! Note:
//! ```ignore
//! s could be empty and contains only lowercase letters a-z.
//! p could be empty and contains only lowercase letters a-z, and characters like . or *.
//! ```
//! 
//! ## Example 1:
//! ```ignore
//! Input:
//! s = "aa"
//! p = "a"
//! Output: false
//! Explanation: "a" does not match the entire string "aa".
//! ```
//!
//! ## Example 2:
//! ```ignore
//! Input:
//! s = "aa"
//! p = "a*"
//! Output: true
//! Explanation: '*' means zero or more of the precedeng element, 'a'.
//! Therefore, by repeating 'a' once, it becomes "aa".
//! ```
//! 
//! ## Example 3:
//! ```ignore
//! Input:
//! s = "ab"
//! p = ".*"
//! Output: true
//! Explanation: ".*" means "zero or more (*) of any character (.)".
//! ```
//! 
//! ## Example 4:
//! ```ignore
//! Input:
//! s = "aab"
//! p = "c*a*b"
//! Output: true
//! Explanation: c can be repeated 0 times, a can be repeated 1 time. Therefore it matches "aab".
//! ```
//! 
//! ## Example 5:
//! ```ignore
//! Input:
//! s = "mississippi"
//! p = "mis*is*p*."
//! Output: false
//! ```
//! 


#[derive(Debug, Clone)]
pub struct Input {
    pub s: String,
    pub p: String,
}
pub type Output = bool;

pub trait Solution {
    fn is_match(&self, s: String, p: String) -> bool;
}

// -----------------------------------------------------------------------------
// Approach 0
pub struct Solution0;
impl Solution for Solution0 {
    fn is_match(&self, s: String, p: String) -> bool {
        
        let s: Vec<char> = s.chars().collect();
        let p: Vec<char> = p.chars().collect();

        is_match_mirror(&s, 0, &p, 0)
    }
}

fn is_match_mirror(s: &[char], s_start: usize, p: &[char], p_start: usize) -> bool {

    match (s_start == s.len(), p_start == p.len()) {
        | (true,  true)  => return true,
        | (false, true)  => return false,
        | (true,  false) => {
            return match p[p_start] {
                | '*' => is_match_mirror(s, s_start, p, p_start + 1),
                | _ => {
                    if p_start + 1 < p.len() && p[p_start + 1] == '*' {
                        is_match_mirror(s, s_start, p, p_start + 2)
                    } else {
                        false
                    }
                },
            }
        },
        | (false, false) => {},
    }
    
    match p[p_start] {
        | '*' => {
            if s[s_start] == p[p_start - 1] || p[p_start - 1] == '.' {
                is_match_mirror(s, s_start + 1, p, p_start) ||
                is_match_mirror(s, s_start, p, p_start + 1)
            } else {
                is_match_mirror(s, s_start, p, p_start + 1)
            }
        },
        | ch if ch == '.' || ch == s[s_start] => {
            is_match_mirror(s, s_start + 1, p, p_start + 1) || {
                if p_start + 1 < p.len() && p[p_start + 1] == '*' {
                    is_match_mirror(s, s_start, p, p_start + 2)
                } else {
                    false
                }
            }
        },
        | _ if p_start + 1 < p.len() && p[p_start + 1] == '*' => {
            is_match_mirror(s, s_start, p, p_start + 2)
        },
        | _ => false,
    }
}
// -----------------------------------------------------------------------------