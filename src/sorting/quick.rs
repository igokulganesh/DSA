fn partision<T: Ord>(arr: &mut [T]) -> usize {
    let pivot = arr.len() -1; 
    let mut i = 0; 

    for j in 0..arr.len()-1 {
        if arr[j] <= arr[pivot] {
            arr.swap(j, i);
            i += 1;
        }
    }

    arr.swap(i, pivot);
    i
}

pub fn quick_sort<T: Ord>(arr: &mut [T]) {
    if arr.len() > 1 {
        let pivot = partision(arr);
        quick_sort(&mut arr[0..pivot]);
        quick_sort(&mut arr[pivot+1..]);
    } 
}

#[test]
fn test_quick_sort_i32() {
    let mut numbers = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    let mut sorted_numbers = numbers.clone();
    sorted_numbers.sort();
    quick_sort(&mut numbers);
    assert_eq!(numbers, sorted_numbers);
}

#[test]
fn test_quick_sort_string() {
    let mut strings = ["apple", "cherry", "banana", "date", "fig"];
    let mut sorted_strings = strings.clone();
    sorted_strings.sort();
    quick_sort(&mut strings);
    assert_eq!(strings, sorted_strings);
}

#[test]
fn test_quick_sort_empty() {
    let mut empty_array: [i32; 0] = [];
    quick_sort(&mut empty_array);
    assert_eq!(empty_array, []);
}

#[test]
fn test_quick_sort_single_element() {
    let mut single_element_array = [1];
    quick_sort(&mut single_element_array);
    assert_eq!(single_element_array, [1]);
}

#[test]
fn test_quick_sort_sorted_array() {
    let mut numbers = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    let mut sorted_numbers = numbers.clone();
    
    sorted_numbers.sort();
    numbers = sorted_numbers.clone();
    
    quick_sort(&mut numbers);
    assert_eq!(numbers, sorted_numbers);
}

#[test]
fn test_quick_sort_decending_array() {
    let mut numbers = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    let mut sorted_numbers = numbers.clone();

    sorted_numbers.sort();
    
    numbers = sorted_numbers.clone();
    numbers.sort();
    numbers.reverse();
    
    quick_sort(&mut numbers);
    assert_eq!(numbers, sorted_numbers);
}

