fn main() {
    let result = max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]);
    assert_eq!(result, 6)
}

pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut dp = nums[0];
    let mut max = nums[0];

    for i in 1..nums.len() {
        if nums[i] > dp + nums[i] {
            dp = nums[i]
        } else {
            dp = dp + nums[i]
        }

        if max < dp {
            max = dp
        }
    }

    max
}
