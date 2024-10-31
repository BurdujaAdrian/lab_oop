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
    pub(crate) coffee_int: Intensity,
    pub(crate) name: String,
}

#[derive(Debug)]
pub struct Americano {
    pub(crate) base: Coffee,
    pub(crate) ml_of_water: u32,
}

#[derive(Debug)]
pub struct Cappuccino {
    pub(crate) base: Coffee,
    pub(crate) ml_of_milk: u32,
}
#[derive(Debug)]
pub struct SyrupCappuccino {
    pub(crate) base: Cappuccino,
    pub(crate) syrup: SyrupType,
}

#[derive(Debug)]
pub struct PuSpLatte {
    pub(crate) base: Cappuccino,
    pub(crate) mg_of_pk_spice: u32,
}
