extern crate rand;


mod leetcode;
mod normal_test;
fn main() {
    let nums: Vec<i32> = vec![3, 2, 4];
    let target = 6;
    let result = leetcode::tencent::two_sum::two_sum(nums, target);
    println!("{:?}", result);

    let nums1 = vec![1, 3];
    let nums2 = vec![2];
    let result = leetcode::tencent::find_median_sorted_arrays::cal(nums1, nums2);
    println!("{:?}", result);

    let nums1 = vec![2, 3, 6, 7];
    let nums2 = vec![1, 5, 9];
    let result = leetcode::tencent::find_median_sorted_arrays::other_cal(nums1, nums2);
    println!("{:?}", result);

    let s = String::from("ababa");
    let result = leetcode::tencent::longest_palindrome::center_expand(s);
    println!("{}", result);

    let s = String::from("   +0 123");
    let result = leetcode::tencent::my_atoi::my_atoi(s);
    println!("{}", result);

    let strs = vec![String::from("flower"),String::from("flow"),String::from("flight")];
    let result = leetcode::tencent::longest_common_prefix::longest_common_prefix(strs);
    println!("{}", result);
}

