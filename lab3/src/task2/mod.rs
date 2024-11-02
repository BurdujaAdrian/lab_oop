#![allow(dead_code, non_snake_case)]

use crate::task1::*;

pub trait CoffeeTrait {
    fn printCoffeeDetails(&self);
}

impl CoffeeTrait for Coffee {
    fn printCoffeeDetails(&self) {
        println!(
            "Name:{}\nCoffee intensity: {:?}",
            self.name, self.coffee_int
        );
    }
}

impl CoffeeTrait for Cappuccino {
    fn printCoffeeDetails(&self) {
        self.base.printCoffeeDetails();
        println!("Cappuccino milk: {} ml", self.ml_of_milk);
    }
}

impl CoffeeTrait for SyrupCappuccino {
    fn printCoffeeDetails(&self) {
        self.base.printCoffeeDetails();
        println!("Syrop added: {:?}", self.syrup);
    }
}

impl CoffeeTrait for PuSpLatte {
    fn printCoffeeDetails(&self) {
        self.base.printCoffeeDetails();
        println!("Spice added:{} mg", self.mg_of_pk_spice);
    }
}

impl CoffeeTrait for Americano {
    fn printCoffeeDetails(&self) {
        self.base.printCoffeeDetails();
        println!("Water added: {} ml", self.ml_of_water);
    }
}
