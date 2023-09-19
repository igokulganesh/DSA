use std::fmt::Debug;

fn merge<T: Ord + Copy + Debug>(arr: &mut [T], start: usize, mid: usize, end: usize) {
    let n1 = mid - start + 1;
    let n2 = end - mid;

    let mut a: Vec<T> = Vec::with_capacity(n1);
    let mut b: Vec<T> = Vec::with_capacity(n2);

    for i in start..=mid {
        a.push(arr[i]);
    }

    for j in mid + 1..=end {
        b.push(arr[j]);
    }

    let mut a_idx = 0;
    let mut b_idx = 0;
    let mut index = start;

    while a_idx < a.len() && b_idx < b.len() {
        if a[a_idx] < b[b_idx] {
            arr[index] = a[a_idx];
            a_idx += 1;
        } else {
            arr[index] = b[b_idx];
            b_idx += 1;
        }
        index += 1;
    }

    while a_idx < a.len() {
        arr[index] = a[a_idx];
        a_idx += 1;
        index += 1;
    }

    while b_idx < b.len() {
        arr[index] = b[b_idx];
        b_idx += 1;
        index += 1;
    }
}

fn merge_sort_util<T: Ord + Copy + Debug>(arr: &mut [T], start: usize, end: usize) {
    if start < end {
        let mid = start + (end - start) / 2;
        merge_sort_util(arr, start, mid);
        merge_sort_util(arr, mid + 1, end);
        merge(arr, start, mid, end);
    }
}

pub fn merge_sort<T: Ord + Copy + Debug>(arr: &mut [T]) {
    if arr.len() == 0 {
        return;
    }
    merge_sort_util(arr, 0, arr.len() - 1);
}

#[test]
fn test_merge_sort_i32() {
    let mut numbers = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    let mut sorted_numbers = numbers.clone();
    sorted_numbers.sort();
    merge_sort(&mut numbers);
    assert_eq!(numbers, sorted_numbers);
}

#[test]
fn test_merge_sort_string() {
    let mut strings = ["apple", "cherry", "banana", "date", "fig"];
    let mut sorted_strings = strings.clone();
    sorted_strings.sort();
    merge_sort(&mut strings);
    assert_eq!(strings, sorted_strings);
}

#[test]
fn test_merge_sort_empty() {
    let mut empty_array: [i32; 0] = [];
    merge_sort(&mut empty_array);
    assert_eq!(empty_array, []);
}

#[test]
fn test_merge_sort_single_element() {
    let mut single_element_array = [1];
    merge_sort(&mut single_element_array);
    assert_eq!(single_element_array, [1]);
}
