#![allow(unused, non_snake_case)]

use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    borrow::BorrowMut,
    cell::{RefCell, RefMut},
    fs, io,
};

fn main() {
    println!("Hello, world!");
    test_semaphore_work();
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

trait Dineable {
    fn serve_dinner(&mut self, car: Car) {
        println!("Serving for car: {:?}", car.id)
    }
}

trait Refuelable {
    fn refuel(&mut self, car: Car) {
        println!("Refuiling car: {:?}", car.id)
    }
}

//
// @enums
//

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
enum CarType {
    ELECTRIC,
    GAS,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
enum PassangerType {
    PEOPLE,
    ROBOTS,
}
//
// @structs
//

struct Semaphore {
    GasStation_forHumans: CarStation,
    GasStation_forRobots: CarStation,
    ElectroStation_forHumans: CarStation,
    ElectroStation_forRobots: CarStation,
}

struct PeopleDinner {
    dined: *mut i32,
    did_not: *mut i32,
}
struct RobotDinner {
    dined: *mut i32,
    did_not: *mut i32,
}
struct ElectricStation {
    refueled: *mut i32,
    consumption: *mut u32,
}
struct GasStation {
    refueled: *mut i32,
    consumption: *mut u32,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
struct Car {
    id: u32,
    r#type: CarType,
    passengers: PassangerType,
    isDining: bool,
    consumption: u32,
}

struct CarStation {
    dining_service: Box<dyn Dineable>,
    refueling_service: Box<dyn Refuelable>,
    queue: Vec<Car>,
}

//
// @implementations
//

// n_who_did_not: i32 = 0;
// n_of_people_who_dined: i32 = 0;
// n_of_robots_who_dined: i32 = 0;
// n_of_electric_cars_refueled: i32 = 0;
// n_of_gas_cars_refueled: i32 = 0;
// electric_cons: u32 = 0;
// gass_cons: u32 = 0;

impl Semaphore {
    fn init(
        n_didnt: &mut i32,
        n_r_dined: &mut i32,
        n_p_dined: &mut i32,
        n_e_refueled: &mut i32,
        n_g_refueled: &mut i32,
        el_cons: &mut u32,
        gas_cons: &mut u32,
    ) -> Self {
        Self {
            GasStation_forHumans: CarStation {
                dining_service: Box::new(PeopleDinner {
                    dined: n_p_dined,
                    did_not: n_didnt,
                }),
                refueling_service: Box::new(GasStation {
                    refueled: n_g_refueled,
                    consumption: gas_cons,
                }),
                queue: vec![],
            },

            GasStation_forRobots: CarStation {
                dining_service: Box::new(RobotDinner {
                    dined: n_r_dined,
                    did_not: n_didnt,
                }),
                refueling_service: Box::new(GasStation {
                    refueled: n_g_refueled,
                    consumption: gas_cons,
                }),
                queue: vec![],
            },

            ElectroStation_forHumans: CarStation {
                dining_service: Box::new(PeopleDinner {
                    dined: n_p_dined,
                    did_not: n_didnt,
                }),
                refueling_service: Box::new(ElectricStation {
                    refueled: n_e_refueled,
                    consumption: el_cons,
                }),
                queue: vec![],
            },
            ElectroStation_forRobots: CarStation {
                dining_service: Box::new(RobotDinner {
                    dined: n_r_dined,
                    did_not: n_didnt,
                }),
                refueling_service: Box::new(ElectricStation {
                    refueled: n_e_refueled,
                    consumption: el_cons,
                }),
                queue: vec![],
            },
        }
    }
    fn work(&mut self) -> io::Result<()> {
        let path = "./queue";

        let entries = fs::read_dir(path)?;

        let file_names: Vec<String> = entries
            .filter_map(|entry| {
                let path = entry.ok()?.path();
                if path.is_file() {
                    path.file_name()?.to_str().map(|s| s.to_owned())
                } else {
                    None
                }
            })
            .collect();

        let cars: Vec<Car> = file_names
            .into_iter()
            .map(|path| {
                println!("path:{}", "./queue/".to_owned() + &path);
                serde_json::from_str(&fs::read_to_string("./queue/".to_owned() + &path).unwrap())
                    .expect("failed to parse json")
            })
            .collect();

        cars.iter().for_each(|car| {
            if car.passengers == PassangerType::PEOPLE {
                if car.r#type == CarType::GAS {
                    self.GasStation_forHumans.add_car(*car);
                    return;
                }
                // else its electric
                self.ElectroStation_forHumans.add_car(*car);
                return;
            }
            // else its for robots
            if car.r#type == CarType::GAS {
                self.GasStation_forRobots.add_car(*car);
                return;
            }
            // else its electric
            self.ElectroStation_forRobots.add_car(*car);
            return;
        });
        println!("{:#?}", cars);
        Ok(())
    }
}

impl CarStation {
    fn serve_cars(&mut self) {
        while let Some(car) = self.queue.deque() {
            self.dining_service.serve_dinner(car);
            self.refueling_service.refuel(car);
        }
    }

    fn add_car(&mut self, car: Car) {
        self.queue.enque(car);
    }
}

impl Dineable for PeopleDinner {
    fn serve_dinner(&mut self, car: Car) {
        unsafe {
            // for simple and dirty shared mutable data
            if car.isDining {
                *self.dined += 1;
            } else {
                *self.did_not += 1;
            }
        }
        println!("Serving people in car: {:?}", car.id)
    }
}

impl Dineable for RobotDinner {
    fn serve_dinner(&mut self, car: Car) {
        unsafe {
            if car.isDining {
                *self.dined += 1;
            } else {
                *self.did_not += 1;
            }
        }
        println!("Serving robots in car: {:?}", car.id)
    }
}

impl Refuelable for ElectricStation {
    fn refuel(&mut self, car: Car) {
        unsafe {
            if car.consumption > 0 {
                *self.refueled += 1;
                *self.consumption += car.consumption;
            }
        }
        println!("Recharging electric car: {:?}", car.id)
    }
}

impl Refuelable for GasStation {
    fn refuel(&mut self, car: Car) {
        unsafe {
            if car.consumption > 0 {
                *self.refueled += 1;
                *self.consumption += car.consumption;
            }
        }
        println!("Refueling gas car: {:?}", car.id)
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
    let mut n_who_did_not: i32 = 0;
    let mut people_diner1 = PeopleDinner {
        dined: &mut n_of_people_who_dined,
        did_not: &mut n_who_did_not,
    };
    let mut people_diner2 = PeopleDinner {
        dined: &mut n_of_people_who_dined,
        did_not: &mut n_who_did_not,
    };

    let car1 = Car {
        id: 1,
        r#type: CarType::GAS,
        passengers: PassangerType::PEOPLE,
        isDining: true,
        consumption: 0,
    };
    let car2 = Car {
        id: 2,
        r#type: CarType::ELECTRIC,
        passengers: PassangerType::PEOPLE,
        isDining: false,
        consumption: 0,
    };

    people_diner1.serve_dinner(car1);
    people_diner2.serve_dinner(car2);

    assert_eq!(n_of_people_who_dined, 1);

    let mut n_of_robots_who_dined: i32 = 0;
    let mut robot_diner1 = RobotDinner {
        dined: &mut n_of_robots_who_dined,
        did_not: &mut n_who_did_not,
    };
    let mut robot_diner2 = RobotDinner {
        dined: &mut n_of_robots_who_dined,
        did_not: &mut n_who_did_not,
    };

    let car3 = Car {
        id: 3,
        r#type: CarType::GAS,
        passengers: PassangerType::ROBOTS,
        isDining: false,
        consumption: 0,
    };

    let car4 = Car {
        id: 4,
        r#type: CarType::GAS,
        passengers: PassangerType::ROBOTS,
        isDining: true,
        consumption: 0,
    };

    robot_diner1.serve_dinner(car3);
    robot_diner2.serve_dinner(car4);

    assert_eq!(n_of_robots_who_dined, 1);

    assert_eq!(n_who_did_not, 2);
}

#[test]
fn test_refueling() {
    let mut n_of_electric_cars_refueled: i32 = 0;
    let mut electric_cons: u32 = 0;
    let mut electric_station1 = ElectricStation {
        refueled: &mut n_of_electric_cars_refueled,
        consumption: &mut electric_cons,
    };
    let mut electric_station2 = ElectricStation {
        refueled: &mut n_of_electric_cars_refueled,
        consumption: &mut electric_cons,
    };

    let car1 = Car {
        id: 1,
        r#type: CarType::ELECTRIC,
        passengers: PassangerType::PEOPLE,
        isDining: false,
        consumption: 0,
    };

    let car2 = Car {
        id: 2,
        r#type: CarType::ELECTRIC,
        passengers: PassangerType::ROBOTS,
        isDining: false,
        consumption: 10,
    };

    electric_station1.refuel(car1);
    electric_station2.refuel(car2);

    assert_eq!(n_of_electric_cars_refueled, 1);

    let mut n_of_gas_cars_refueled: i32 = 0;
    let mut gass_cons: u32 = 0;
    let mut gas_station1 = GasStation {
        refueled: &mut n_of_gas_cars_refueled,
        consumption: &mut gass_cons,
    };

    let car3 = Car {
        id: 3,
        r#type: CarType::GAS,
        passengers: PassangerType::PEOPLE,
        isDining: false,
        consumption: 0,
    };

    gas_station1.refuel(car3);

    assert_eq!(n_of_gas_cars_refueled, 0);

    let mut gas_station2 = GasStation {
        refueled: &mut n_of_gas_cars_refueled,
        consumption: &mut gass_cons,
    };

    let car4 = Car {
        id: 4,
        r#type: CarType::GAS,
        passengers: PassangerType::PEOPLE,
        isDining: false,
        consumption: 20,
    };

    gas_station2.refuel(car4);

    assert_eq!(n_of_gas_cars_refueled, 1);

    assert_eq!(gass_cons, 20);
    assert_eq!(electric_cons, 10);
}

#[test]
fn test_serving() {
    let mut n_who_did_not: i32 = 0;
    let mut n_of_people_who_dined: i32 = 0;
    let mut n_of_robots_who_dined: i32 = 0;
    let mut n_of_electric_cars_refueled: i32 = 0;
    let mut n_of_gas_cars_refueled: i32 = 0;
    let mut electric_cons: u32 = 0;
    let mut gass_cons: u32 = 0;

    let car_queue = vec![
        Car {
            id: 1,
            r#type: CarType::ELECTRIC,
            passengers: PassangerType::PEOPLE,
            isDining: true,
            consumption: 10,
        },
        Car {
            id: 2,
            r#type: CarType::GAS,
            passengers: PassangerType::PEOPLE,
            isDining: true,
            consumption: 10,
        },
        Car {
            id: 3,
            r#type: CarType::ELECTRIC,
            passengers: PassangerType::ROBOTS,
            isDining: true,
            consumption: 10,
        },
        Car {
            id: 4,
            r#type: CarType::GAS,
            passengers: PassangerType::ROBOTS,
            isDining: true,
            consumption: 10,
        },
        Car {
            id: 5,
            r#type: CarType::GAS,
            passengers: PassangerType::PEOPLE,
            isDining: false,
            consumption: 10,
        },
        Car {
            id: 6,
            r#type: CarType::GAS,
            passengers: PassangerType::ROBOTS,
            isDining: false,
            consumption: 0,
        },
        Car {
            id: 7,
            r#type: CarType::ELECTRIC,
            passengers: PassangerType::PEOPLE,
            isDining: false,
            consumption: 0,
        },
        Car {
            id: 8,
            r#type: CarType::ELECTRIC,
            passengers: PassangerType::ROBOTS,
            isDining: true,
            consumption: 0,
        },
    ];

    let mut robo_electric_station = CarStation {
        dining_service: Box::new(RobotDinner {
            dined: &mut n_of_robots_who_dined,
            did_not: &mut n_who_did_not,
        }),
        refueling_service: Box::new(ElectricStation {
            refueled: &mut n_of_electric_cars_refueled,
            consumption: &mut electric_cons,
        }),
        queue: vec![car_queue[2], car_queue[7]],
    };
    let mut robo_gas_station = CarStation {
        dining_service: Box::new(RobotDinner {
            dined: &mut n_of_robots_who_dined,
            did_not: &mut n_who_did_not,
        }),
        refueling_service: Box::new(GasStation {
            refueled: &mut n_of_gas_cars_refueled,
            consumption: &mut gass_cons,
        }),
        queue: vec![car_queue[3], car_queue[5]],
    };

    let mut human_gas_station = CarStation {
        dining_service: Box::new(PeopleDinner {
            dined: &mut n_of_people_who_dined,
            did_not: &mut n_who_did_not,
        }),
        refueling_service: Box::new(GasStation {
            refueled: &mut n_of_gas_cars_refueled,
            consumption: &mut gass_cons,
        }),
        queue: vec![car_queue[1], car_queue[4]],
    };

    let mut human_electric_station = CarStation {
        dining_service: Box::new(PeopleDinner {
            dined: &mut n_of_people_who_dined,
            did_not: &mut n_who_did_not,
        }),
        refueling_service: Box::new(ElectricStation {
            refueled: &mut n_of_electric_cars_refueled,
            consumption: &mut electric_cons,
        }),
        queue: vec![car_queue[0], car_queue[6]],
    };

    robo_electric_station.serve_cars();

    human_gas_station.serve_cars();

    robo_gas_station.serve_cars();

    human_electric_station.serve_cars();

    assert_eq!(n_who_did_not, 3);
    assert_eq!(n_of_people_who_dined, 2);
    assert_eq!(n_of_robots_who_dined, 3);
    assert_eq!(n_of_electric_cars_refueled, 2);
    assert_eq!(n_of_gas_cars_refueled, 3);
    assert_eq!(electric_cons, 20);
    assert_eq!(gass_cons, 30);
}

//#[test]
fn test_semaphore_work() {
    let mut n_who_did_not: i32 = 0;
    let mut n_of_people_who_dined: i32 = 0;
    let mut n_of_robots_who_dined: i32 = 0;
    let mut n_of_electric_cars_refueled: i32 = 0;
    let mut n_of_gas_cars_refueled: i32 = 0;
    let mut electric_cons: u32 = 0;
    let mut gas_cons: u32 = 0;

    let mut semaphore = Semaphore::init(
        &mut n_who_did_not,
        &mut n_of_robots_who_dined,
        &mut n_of_people_who_dined,
        &mut n_of_electric_cars_refueled,
        &mut n_of_gas_cars_refueled,
        &mut electric_cons,
        &mut gas_cons,
    );

    semaphore.work();
    //    panic!("end the test");
}

//@end of tests
