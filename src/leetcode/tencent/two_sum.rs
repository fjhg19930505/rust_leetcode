/*给定一个整数数组 nums 和一个目标值 target，请你在该数组中找出和为目标值的那 两个 整数，并返回他们的数组下标。

你可以假设每种输入只会对应一个答案。但是，你不能重复利用这个数组中同样的元素。

示例:

给定 nums = [2, 7, 11, 15], target = 9

因为 nums[0] + nums[1] = 2 + 7 = 9
所以返回 [0, 1]
*/

use std::collections::HashMap;
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[i] + nums[j] == target {
                res.push(i as i32);
                res.push(j as i32);
                return res;
            }
        }
    }
    return res;
}

pub fn best_two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut myhash:HashMap<i32, i32> = HashMap::new();

    for (index, &num) in nums.iter().enumerate() {
        let target_to_check = target - num;
        if myhash.contains_key(&target_to_check) {
            return vec![*myhash.get(&target_to_check).unwrap(), index as i32];
        }
        myhash.insert(num,index as i32);
    }
    return vec![];
}