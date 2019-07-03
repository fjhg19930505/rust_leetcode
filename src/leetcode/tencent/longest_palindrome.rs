/**
最长回文子串
给定一个字符串 s，找到 s 中最长的回文子串。你可以假设 s 的最大长度为 1000。

示例 1：

输入: "babad"
输出: "bab"
注意: "aba" 也是一个有效答案。
示例 2：

输入: "cbbd"
输出: "bb"
 */
use std::cmp;
pub fn longest_palindrome(s: String) -> String {
    let mut max = 0;
    let mut max_str = String::new();
    for i in 0..s.len() {
        for j in (i + 1..s.len() + 1).rev() {

            if max_str.len() >= s.len() - i {
                return max_str;
            }

            if max_str.len() >= j - i {
                break;
            }

            let temp = &s[i..j];
            let bytes = temp.as_bytes();
            let mut bytes = Vec::from(bytes);
            bytes.reverse();
            let temp_rev = String::from_utf8(bytes).unwrap();
            if temp == temp_rev && temp.len() > max {
                max_str = String::from(temp);
                max = temp.len();
            }
        }
    }

    if s.is_empty() {
        return String::from("");
    }
    return String::from(max_str);
}

// 中心扩展法
pub fn center_expand(s: String) -> String {
    if s.is_empty() {
        return String::from("");
    }

    let mut start = 0;
    let mut end = 0;
    for i in 0..s.len() {
        // 奇数扩展
        let len1 = expand_around_center(&s, i, i);
        // 偶数扩展
        let len2 = expand_around_center(&s, i, i + 1);
        let len = cmp::max(len1, len2);
        if len > end - start {
            start = i - (len - 1) / 2;
            end = i + len / 2;
        }
    }

    return String::from(&s[start..end + 1]);
}

fn expand_around_center(s: &String, left: usize, right: usize) -> usize {
    let mut l = left as i32;
    let mut r = right as i32;

    while l >= 0
        && r < s.len() as i32
        && &s[l as usize..(l + 1) as usize] == &s[r as usize..(r + 1) as usize]
    {
        l -= 1;
        r += 1;
    }

    return (r - l - 1) as usize;
}