pub fn sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        for j in 0..arr.len()-i-1 {
            if arr[j] > arr[j+1] {
                arr.swap(j, j+1);
            }
        }
    }
}



#[test]
fn bubble_sort_test() {
    let mut numbers = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    let mut sorted_numbers = numbers.clone();
    sorted_numbers.sort();

    sort(&mut numbers);

    assert_eq!(numbers, sorted_numbers);

    let mut strings = ["apple", "cherry", "banana", "date", "fig"];
    let mut sorted_strings = strings.clone();
    sorted_strings.sort();
    sort(&mut strings);
    assert_eq!(strings, sorted_strings);
}