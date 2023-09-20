pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        for j in 0..arr.len() - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

#[test]
fn test_bubble_sort_i32() {
    let mut numbers = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    let mut sorted_numbers = numbers.clone();
    sorted_numbers.sort();
    bubble_sort(&mut numbers);
    assert_eq!(numbers, sorted_numbers);
}

#[test]
fn test_bubble_sort_string() {
    let mut strings = ["apple", "cherry", "banana", "date", "fig"];
    let mut sorted_strings = strings.clone();
    sorted_strings.sort();
    bubble_sort(&mut strings);
    assert_eq!(strings, sorted_strings);
}

#[test]
fn test_bubble_sort_empty() {
    let mut empty_array: [i32; 0] = [];
    bubble_sort(&mut empty_array);
    assert_eq!(empty_array, []);
}

#[test]
fn test_bubble_sort_single_element() {
    let mut single_element_array = [1];
    bubble_sort(&mut single_element_array);
    assert_eq!(single_element_array, [1]);
}


#[test]
fn test_bubble_sort_sorted_array() {
    let mut numbers = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    let mut sorted_numbers = numbers.clone();
    
    sorted_numbers.sort();
    numbers = sorted_numbers.clone();
    
    bubble_sort(&mut numbers);
    assert_eq!(numbers, sorted_numbers);
}

#[test]
fn test_bubble_sort_decending_array() {
    let mut numbers = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    let mut sorted_numbers = numbers.clone();

    sorted_numbers.sort();
    
    numbers = sorted_numbers.clone();
    numbers.sort();
    numbers.reverse();
    
    bubble_sort(&mut numbers);
    assert_eq!(numbers, sorted_numbers);
}
