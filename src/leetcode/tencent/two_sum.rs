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