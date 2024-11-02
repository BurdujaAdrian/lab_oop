#![allow(non_snake_case)]

use crate::task1::*;

impl Coffee {
    pub fn makeCoffee(coffee_int: Intensity) -> Coffee {
        Coffee {
            name: "coffee".to_string(),
            coffee_int,
        }
    }
}

impl Cappuccino {
    pub fn makeCappuccino(coffee_int: Intensity, ml_of_milk: u32) -> Cappuccino {
        Cappuccino {
            base: Coffee {
                name: "Cappuccino".to_string(),
                coffee_int,
            },
            ml_of_milk,
        }
    }
}

impl SyrupCappuccino {
    pub fn makeSyrupCappuccino(
        coffee_int: Intensity,
        ml_of_milk: u32,
        syrup: SyrupType,
    ) -> SyrupCappuccino {
        SyrupCappuccino {
            base: Cappuccino {
                base: Coffee {
                    name: "SyrupCoffee".to_string(),
                    coffee_int,
                },
                ml_of_milk,
            },
            syrup,
        }
    }
}

impl PuSpLatte {
    pub fn makePuSpLatte(coffee_int: Intensity, ml_of_milk: u32, mg_of_pk_spice: u32) -> PuSpLatte {
        PuSpLatte {
            base: Cappuccino {
                base: Coffee {
                    name: "PumpkinSpiceLatte".to_string(),
                    coffee_int,
                },
                ml_of_milk,
            },
            mg_of_pk_spice,
        }
    }
}

impl Americano {
    pub fn makeAmericano(
        coffee_int: Intensity,
        ml_of_water: u32, //
    ) -> Americano {
        Americano {
            base: Coffee {
                name: "Americano".to_string(),
                coffee_int,
            },
            ml_of_water,
        }
    }
}
