#![allow(unused)]

use std::{
    borrow::BorrowMut,
    cell::{RefCell, RefMut},
};
fn main() {
    println!("Hello, world!");
    todo!("Write tests for new traits")
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
    fn serve_dinner(&mut self, car_id: String) {
        println!("Serving for car: {car_id}")
    }
}

pub trait Refuelable {
    fn refuel(&mut self, car_id: String) {
        println!("Refuiling car: {car_id}")
    }
}

//
// @structs
//

struct PeopleDinner {
    dined: *mut i32,
}
struct RobotDinner {
    dined: *mut i32,
}
struct ElectricStation {
    refueled: *mut i32,
}
struct GasStation {
    refueled: *mut i32,
}

//
// @implementations
//

impl Dineable for PeopleDinner {
    fn serve_dinner(&mut self, car_id: String) {
        unsafe {
            *self.dined += 1;
        }
        println!("Serving people in car: {car_id}")
    }
}

impl Dineable for RobotDinner {
    fn serve_dinner(&mut self, car_id: String) {
        unsafe {
            *self.dined += 1;
        }
        println!("Serving robots in car: {car_id}")
    }
}

impl Refuelable for ElectricStation {
    fn refuel(&mut self, car_id: String) {
        unsafe {
            *self.refueled += 1;
        }
        println!("Recharging electric car: {car_id}")
    }
}

impl Refuelable for GasStation {
    fn refuel(&mut self, car_id: String) {
        unsafe {
            *self.refueled += 1;
        }
        println!("Refueling gas car: {car_id}")
    }
}

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

#[test]
fn test_dining() {
    let mut n_of_people_who_dined: i32 = 0;
    let mut people_diner1 = PeopleDinner {
        dined: &mut n_of_people_who_dined,
    };
    let mut people_diner2 = PeopleDinner {
        dined: &mut n_of_people_who_dined,
    };

    people_diner2.serve_dinner("1".to_string());
    people_diner1.serve_dinner("2".to_string());

    assert_eq!(n_of_people_who_dined, 2);

    let mut n_of_robots_who_dined: i32 = 0;
    let mut robot_diner1 = RobotDinner {
        dined: &mut n_of_robots_who_dined,
    };

    let mut robot_diner2 = RobotDinner {
        dined: &mut n_of_robots_who_dined,
    };
    let mut robot_diner3 = RobotDinner {
        dined: &mut n_of_robots_who_dined,
    };

    robot_diner1.serve_dinner("3".to_string());
    robot_diner3.serve_dinner("1".to_string());
    robot_diner2.serve_dinner("2".to_string());

    assert_eq!(n_of_robots_who_dined, 3);
}

#[test]
fn test_refueling() {
    let mut n_of_electric_cars_refueled: i32 = 0;
    let mut electric_station1 = ElectricStation {
        refueled: &mut n_of_electric_cars_refueled,
    };
    let mut electric_station2 = ElectricStation {
        refueled: &mut n_of_electric_cars_refueled,
    };

    electric_station1.refuel("ElectricCar1".to_string());
    electric_station2.refuel("ElectricCar2".to_string());

    assert_eq!(n_of_electric_cars_refueled, 2);

    let mut n_of_gas_cars_refueled: i32 = 0;
    let mut gas_station1 = GasStation {
        refueled: &mut n_of_gas_cars_refueled,
    };
    let mut gas_station2 = GasStation {
        refueled: &mut n_of_gas_cars_refueled,
    };
    let mut gas_station3 = GasStation {
        refueled: &mut n_of_gas_cars_refueled,
    };

    gas_station1.refuel("GasCar1".to_string());
    gas_station3.refuel("GasCar3".to_string());
    gas_station2.refuel("GasCar2".to_string());

    assert_eq!(n_of_gas_cars_refueled, 3);
}

//@end of tests
