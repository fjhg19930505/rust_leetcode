extern crate math;
use std::cmp;

pub fn cal(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut m = nums1.len();
    let mut n = nums2.len();

    // 如果第一个数组的长度比第二个长，交换
    let mut a: Vec<i32> = nums1;
    let mut b: Vec<i32> = nums2;
    if m > n {
        let temp = a;
        a = b;
        b = temp;
        let tep = m;
        m = n;
        n = tep;
    }

    let mut min = 0;
    let mut max = m;
    let half_len = (m + n + 1) / 2;
    while min <= max {
        let i = (min + max) / 2;
        let j = half_len - i;

        if i < max && b[j - 1] > a[i] {
            min = i + 1;
        } else if i > min && a[i - 1] > b[j] {
            max = i - 1
        } else {
            let mut max_left: f64 = 0.0;
            if i == 0 {
                max_left = b[j - 1] as f64;
            } else if j == 0 {
                max_left = a[i - 1] as f64;
            } else {
                max_left = cmp::max(a[i - 1], b[j - 1]) as f64;
            }

            if (m + n) % 2 == 1 {
                return max_left;
            }

            let mut min_right: f64 = 0.0;
            if i == m {
                min_right = b[j] as f64;
            } else if j == n {
                min_right = a[i] as f64;
            } else {
                min_right = cmp::min(b[j], a[i]) as f64;
            }

            return (max_left + min_right) / 2.0;
        }
    }

    return 0.0;
}

pub fn other_cal(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut m = nums1.len();
    let mut n = nums2.len();

    // 如果第一个数组的长度比第二个长，交换
    let mut a: Vec<i32> = nums1;
    let mut b: Vec<i32> = nums2;
    if m > n {
        let temp = a;
        a = b;
        b = temp;
        let tep = m;
        m = n;
        n = tep;
    }

    let mut lo = 0;
    let mut hi = 2 * m;
    let mut l_max1 = 0;
    let mut r_min1 = 0;
    let mut l_max2 = 0;
    let mut r_min2 = 0;
    // 二分
    while lo <= hi {
        let c1 = (lo + hi) / 2; // 二分的记过
        let c2 = m + n - c1;

        l_max1 = if c1 == 0 {
            std::i32::MIN
        } else {
            a[(c1 - 1) / 2]
        };

        r_min1 = if c1 == 2 * m {
            std::i32::MAX
        } else {
            a[c1 / 2]
        };

        l_max2 = if c2 == 0 {
            std::i32::MIN
        } else {
            b[(c2 - 1) / 2]
        };

        r_min2 = if c2 == 2 * n {
            std::i32::MAX
        } else {
            b[c2 / 2]
        };

        if l_max1 > r_min2 {
            hi = c1 - 1;
        } else if l_max2 > r_min1 {
            lo = c1 + 1;
        } else {
            break;
        }

    }

    return ((cmp::max(l_max1, l_max2) + cmp::min(r_min1, r_min2)) as f64) / 2.0;
}