#[test]
fn test_list_push_front() {
    use super::List;

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
    use super::List;

    let mut list = List::new();

    for i in 1..=100 {
        list.push_back(i);
    }

    let list_vec: Vec<i32> = list.into();

    let check_vec: Vec<i32> = (1..=100).collect();

    assert_eq!(check_vec, list_vec);
}

#[test]
fn test_list_size() {
    use super::List;

    let mut list = List::new();

    assert!(list.is_empty());

    for i in 1..=100 {
        list.push_front(i);
        assert_eq!(i, list.len());
    }

    assert!(!list.is_empty());
}

#[test]
fn test_list_pop_front() {
    use super::List;

    let mut list = List::new();

    for i in 1..=100 {
        list.push_back(i);
    }

    let mut data;
    for i in 1..=100 {
        data = list.pop_front();
        assert_eq!(i, data.unwrap());
    }

    assert_eq!(None, list.pop_front());
}

#[test]
fn test_list_pop_back() {
    use super::List;

    let mut list = List::new();

    for i in 1..=100 {
        list.push_front(i);
    }

    let mut data;
    for i in 1..=100 {
        data = list.pop_back();
        assert_eq!(i, data.unwrap());
    }

    assert_eq!(None, list.pop_back());
}

#[test]
fn test_doubly_list_push_front() {
    use super::DoublyList;

    let mut list = DoublyList::new();

    for i in 1..=100 {
        list.push_front(i);
    }

    let list_vec: Vec<i32> = list.into();

    let check_vec: Vec<i32> = (1..=100).rev().collect();

    assert_eq!(check_vec, list_vec);
}

#[test]
fn test_doubly_list_push_back() {
    use super::DoublyList;

    let mut list = DoublyList::new();

    for i in 1..=100 {
        list.push_back(i);
    }

    let list_vec: Vec<i32> = list.into();

    let check_vec: Vec<i32> = (1..=100).collect();

    assert_eq!(check_vec, list_vec);
}

#[test]
fn test_doubly_list_pop_front() {
    use super::DoublyList;

    let mut list = DoublyList::new();

    for i in 1..=100 {
        list.push_back(i);
    }

    let mut data;
    for i in 1..=100 {
        data = list.pop_front();
        assert_eq!(i, data.unwrap());
    }

    assert_eq!(None, list.pop_front());
}