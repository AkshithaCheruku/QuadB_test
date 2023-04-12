fn find_first_occurrence(arr: &[i32], x: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left <= right {
        let mid = (left + right) / 2;
        if arr[mid] < x {
            left = mid + 1;
        } else if arr[mid] > x {
            right = mid - 1;
        } else {
            // found the element
            if mid == 0 || arr[mid-1] != x {
                return Some(mid);
            } else {
                right = mid - 1;
            }
        }
    }

    None
}

fn main() {
    let arr = [1, 3, 4, 4, 4, 6, 8];
    let x1 = 4;
    let x2 = 7;
    match find_first_occurrence(&arr, x1) {
        Some(index) => println!("The first occurrence of {} is at index {}", x1, index),
        None => println!("{} not found in the array", x1),
    }
    match find_first_occurrence(&arr, x2) {
        Some(index) => println!("The first occurrence of {} is at index {}", x2, index),
        None => println!("{} not found in the array", x2),
    }
}
