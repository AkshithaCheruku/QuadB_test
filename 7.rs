fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    if k > arr.len() {
        return None;
    }

    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort();

    Some(sorted_arr[k-1])
}

fn main() {
    let arr1 = [3, 1, 4, 2, 5];
    let arr2 = [10, 7, 8, 9, 6];
    let k1 = 2;
    let k2 = 4;
    println!("{}th smallest element in {:?}: {:?}", k1, arr1, kth_smallest(&arr1, k1));
    println!("{}th smallest element in {:?}: {:?}", k2, arr2, kth_smallest(&arr2, k2));
}
