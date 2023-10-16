#[test]
fn test_list() {
    use super::list::List;

    let mut list = List::new();

    for i in 1..=100 {
        list.push_front(i);
    }

    let list_vec: Vec<i32> = list.into();

    let check_vec: Vec<i32> = (1..=100).rev().collect();

    assert_eq!(check_vec, list_vec);
}

#[test]
fn test_list_push_back() {
    use super::list::List;

    let mut list = List::new();

    for i in 1..=100 {
        list.push_back(i);
    }

    let list_vec: Vec<i32> = list.into();

    let check_vec: Vec<i32> = (1..=100).collect();

    assert_eq!(check_vec, list_vec);
}
