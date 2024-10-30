#![allow(dead_code)]

#[derive(Debug)]
pub enum Intensity {
    LIGHT,
    NORMAL,
    STRONG,
}

#[repr(C)]
#[derive(Debug)]
pub enum SyrupType {
    MACADAMIA,
    VANILLA,
    COCONUT,
    CARAMEL,
    CHOCOLATE,
    POPCORN,
}

#[derive(Debug)]
pub struct Coffee {
    coffee_int: Intensity,
    name: String,
}

#[derive(Debug)]
pub struct Americano {
    base: Coffee,
    ml_of_water: u32,
}

#[derive(Debug)]
pub struct Cappucino {
    base: Coffee,
    ml_of_milk: u32,
}
#[derive(Debug)]
pub struct SyrupCappuccino {
    base: Cappucino,
    syrup: SyrupType,
}

#[derive(Debug)]
pub struct PuSpLatte {
    base: Cappucino,
    mg_of_pk_spice: u16,
}
