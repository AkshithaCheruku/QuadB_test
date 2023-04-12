fn find_median(nums: &[i32]) -> f64 {
    let n = nums.len();
    if n % 2 == 0 {
        let i = n/2 - 1;
        let j = n/2;
        (nums[i] + nums[j]) as f64 / 2.0
    } else {
        let i = n/2;
        nums[i] as f64
    }
}

fn main() {
    let nums1 = vec![1, 2, 3, 4, 5];
    let nums2 = vec![1, 2, 3, 4, 5, 6];
    let nums3 = vec![1, 2, 3, 4, 5, 6, 7];
    println!("Median of {:?}: {}", nums1, find_median(&nums1));
    println!("Median of {:?}: {}", nums2, find_median(&nums2));
    println!("Median of {:?}: {}", nums3, find_median(&nums3));
}
