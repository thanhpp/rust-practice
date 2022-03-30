fn main() {
    println!("Hello, world!");
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
