#[test]
fn test_stack_with_some() {
    use super::stack::Stack;

    let mut s = Stack::new();

    for i in 1..=100 {
        s.push(i);
    }

    for i in (1..=100).rev() {
        if let Some(val) = s.pop() {
            assert_eq!(i, val);
        } else {
            panic!("Expected Some Value, Got None Value");
        }
    }
}

#[test]
fn test_stack_with_none() {
    use super::stack::Stack;

    let mut s: Stack<i32> = Stack::new();

    // expecting None Value
    for _ in 0..10 {
        match s.pop() {
            None => (),
            Some(val) => panic!("Expected None Value, Got Some Value {}", val),
        }
    }
}
