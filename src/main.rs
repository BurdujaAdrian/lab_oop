fn main() {
    println!("Hello, world!");
}

pub trait Queue {
    type Item;
    fn enque(&mut self, item: Self::Item);
    fn deque(&mut self) -> Option<Self::Item>;
    fn rear(&self) -> Option<&Self::Item>;
    fn front(&self) -> Option<&Self::Item>;
}

impl Queue for Vec<i32> {
    type Item = i32;

    fn enque(&mut self, item: i32) {
        self.push(item);
    }

    fn deque(&mut self) -> Option<i32> {
        if self.len() == 0 {
            return None;
        }
        let res = self[0];
        self.remove(0);

        Some(res)
    }

    fn rear(&self) -> Option<&i32> {
        return self.last();
    }

    fn front(&self) -> Option<&Self::Item> {
        return self.first();
    }
}

#[test]
fn test_tests() {
    assert!(true)
}

#[test]
fn test_queue_i32() {
    let mut queue: Vec<i32> = vec![];

    // enque works as expected
    queue.enque(1);
    queue.enque(2);

    assert_eq!(queue, vec![1, 2]);

    // rear works as expected

    assert_eq!(queue.rear(), Some(&2));
    assert_eq!(queue.front(), Some(&1));

    // deque works as expected
    assert_eq!(queue.deque(), Some(1));
    println!("{:?}", queue);
    assert_eq!(queue.deque(), Some(2));
    println!("{:?}", queue);
    assert_eq!(queue.deque(), None);

    assert_eq!(queue.rear(), None);
    assert_eq!(queue.front(), None);
}
