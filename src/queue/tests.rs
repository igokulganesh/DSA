#[test]
fn test_queue_with_some() {
    use super::queue::Queue;

    let mut que = Queue::new();

    for i in 1..=100 {
        que.push(i);
    }

    for i in 1..=100 {
        if let Some(val) = que.pop() {
            assert_eq!(i, val);
        } else {
            panic!("Expected Some Value, Got None Value");
        }
    }
}

#[test]
fn test_queue_with_none() {
    use super::queue::Queue;

    let mut que: Queue<i32> = Queue::new();

    for _ in 1..=100 {
        match que.pop() {
            Some(val) => panic!("Expected None Value, Got Some Value {}", val),
            None => (),  
        }
    }
}