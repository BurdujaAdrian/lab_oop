#![allow(unused)]

fn main() {
    println!("Hello, world!");
}

//
// @traits
//

pub trait Queue {
    type Item;
    fn enque(&mut self, item: Self::Item);
    fn deque(&mut self) -> Option<Self::Item>;
    fn rear(&self) -> Option<&Self::Item>;
    fn front(&self) -> Option<&Self::Item>;
}

pub trait Dineable {
    fn serve_dinner(car_id: String);
}

pub trait Refuelable {
    fn refuel(car_id: String);
}

//
// @structs
//

struct PeopleDinner {}
struct RobotDinner {}
struct ElectricStation {}
struct GasStation {}

//
// @implementations
//

/*
impl Queue for Vec<i32> {
    type Item = i32;

    fn enque(&mut self, item: Self::Item) {
        self.push(item);
    }

    fn deque(&mut self) -> Option<Self::Item> {
        if self.len() == 0 {
            return None;
        }
        let res = self[0].clone();
        self.remove(0);

        Some(res)
    }

    fn rear(&self) -> Option<&Self::Item> {
        return self.last();
    }

    fn front(&self) -> Option<&Self::Item> {
        return self.first();
    }
}
*/

/*
impl Queue for Vec<String> {
    type Item = String;

    fn enque(&mut self, item: Self::Item) {
        self.push(item);
    }
    fn deque(&mut self) -> Option<Self::Item> {
        if self.len() == 0 {
            return None;
        }
        let res = self[0].clone();
        self.remove(0);

        Some(res)
    }

    fn rear(&self) -> Option<&Self::Item> {
        return self.last();
    }

    fn front(&self) -> Option<&Self::Item> {
        return self.first();
    }
}
*/

impl<T: Clone> Queue for Vec<T> {
    type Item = T;

    fn enque(&mut self, item: Self::Item) {
        self.push(item);
    }
    fn deque(&mut self) -> Option<Self::Item> {
        if self.len() == 0 {
            return None;
        }
        let res = self[0].clone();
        self.remove(0);

        Some(res)
    }

    fn rear(&self) -> Option<&Self::Item> {
        return self.last();
    }

    fn front(&self) -> Option<&Self::Item> {
        return self.first();
    }
}

//
// @tests
//

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

#[test]
fn test_queue_string() {
    let mut queue: Vec<String> = vec![];

    // enque works as expected
    queue.enque("1".to_string());
    queue.enque("2".to_string());

    assert_eq!(queue, vec!["1", "2"]);

    // rear works as expected

    assert_eq!(queue.rear(), Some(&"2".to_string()));
    assert_eq!(queue.front(), Some(&"1".to_string()));

    // deque works as expected
    assert_eq!(queue.deque(), Some("1".to_string()));
    println!("{:?}", queue);
    assert_eq!(queue.deque(), Some("2".to_string()));
    println!("{:?}", queue);
    assert_eq!(queue.deque(), None);

    assert_eq!(queue.rear(), None);
    assert_eq!(queue.front(), None);
}

#[derive(Clone, PartialEq, Debug)]
struct TestStruct {
    copyable: i32,
    heap_allocated: String,
}

#[test]
fn test_queue_struct() {
    let mut queue: Vec<TestStruct> = vec![];
    let one = TestStruct {
        copyable: 1,
        heap_allocated: "1".to_string(),
    };
    let two = TestStruct {
        copyable: 2,
        heap_allocated: "2".to_string(),
    };
    queue.enque(one.clone());
    queue.enque(two.clone());

    assert_eq!(queue, vec![one.clone(), two.clone()]);

    // rear works as expected

    assert_eq!(queue.rear(), Some(&two));
    assert_eq!(queue.front(), Some(&one));

    // deque works as expected
    assert_eq!(queue.deque(), Some(one));
    println!("{:?}", queue);
    assert_eq!(queue.deque(), Some(two));
    println!("{:?}", queue);
    assert_eq!(queue.deque(), None);

    assert_eq!(queue.rear(), None);
    assert_eq!(queue.front(), None);
}
