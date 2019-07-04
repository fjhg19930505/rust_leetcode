/*
请你来实现一个 atoi 函数，使其能将字符串转换成整数。

首先，该函数会根据需要丢弃无用的开头空格字符，直到寻找到第一个非空格的字符为止。

当我们寻找到的第一个非空字符为正或者负号时，则将该符号与之后面尽可能多的连续数字组合起来，作为该整数的正负号；假如第一个非空字符是数字，则直接将其与之后连续的数字字符组合起来，形成整数。

该字符串除了有效的整数部分之后也可能会存在多余的字符，这些字符可以被忽略，它们对于函数不应该造成影响。

注意：假如该字符串中的第一个非空格字符不是一个有效整数字符、字符串为空或字符串仅包含空白字符时，则你的函数不需要进行转换。

在任何情况下，若函数不能进行有效的转换时，请返回 0。

说明：

假设我们的环境只能存储 32 位大小的有符号整数，那么其数值范围为 [−231,  231 − 1]。如果数值超过这个范围，qing返回  INT_MAX (231 − 1) 或 INT_MIN (−231) 。

示例 1:

输入: "42"
输出: 42
示例 2:

输入: "   -42"
输出: -42
解释: 第一个非空白字符为 '-', 它是一个负号。
     我们尽可能将负号与后面所有连续出现的数字组合起来，最后得到 -42 。
示例 3:

输入: "4193 with words"
输出: 4193
解释: 转换截止于数字 '3' ，因为它的下一个字符不为数字。
示例 4:

输入: "words and 987"
输出: 0
解释: 第一个非空字符是 'w', 但它不是数字或正、负号。
     因此无法执行有效的转换。
示例 5:

输入: "-91283472332"
输出: -2147483648
解释: 数字 "-91283472332" 超过 32 位有符号整数范围。 
     因此返回 INT_MIN (−231) 。
 */

pub fn my_atoi(str: String) -> i32 {
    
    if str.is_empty() {
        return 0;
    }

    // 去除所有空格
    let trim_str = str.trim();
    println!("{}", trim_str);

    if trim_str.is_empty() {
        return 0;
    }
    
    if trim_str.len() == 1 {
        let ch = trim_str.chars().next();
        if ch < Some('0') || ch > Some('9') {
            return 0;
        }
    }

    // 碰到不是加减号和数字，返回最后一个是数字的index
    let mut index = 0;
    let mut has_op = false; // 有没有符号位
    let mut zero_end = 0;
    let mut zero_flag = false;
    for ch in trim_str.chars() { 
        if index == 0 {
            if ch != '+' && ch != '-' && (ch < '0' || ch > '9') {
                return 0;
            } 

            if ch == '+' || ch == '-' {
                has_op = true;
            }
        } else if (ch < '0' || ch > '9'){
            
            break;
        } 

        if !zero_flag {
            if has_op && index != 0 && ch != '0' {
                zero_flag = true;
            }
            else if (!has_op && ch != '0') {
                zero_flag = true;
            }
            else {
                zero_end += 1;
            }

            
        }

        index += 1;
    }

    // 可以格式化成数字的字符串是
    let mut num_str = String::from(&trim_str[0..index]);

    if has_op {
        if zero_end > 1 {
            num_str.replace_range(1..zero_end, "");
        }
    } else {
        if zero_end >= 1{
            num_str.replace_range(0..zero_end, "");
        }
    }
    
    if num_str.is_empty() || num_str == "+" || num_str == "-"{
        return 0;
    }
    
    // 如果大于11
    if num_str.len() > 11 {
        if num_str.chars().next() == Some('-') {
            return std::i32::MIN;
        }
        else {
            return std::i32::MAX;
        }
    }

    let result : i64= num_str.parse().unwrap();
    if result > std::i32::MAX as i64 {
        return std::i32::MAX;
    }
    else if result < std::i32::MIN as i64 {
        return std::i32::MIN;
    }

    return result as i32;
}