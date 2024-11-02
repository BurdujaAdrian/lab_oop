#![allow(dead_code)]

#[derive(Debug)]
pub(crate) enum Intensity {
    LIGHT,
    NORMAL,
    STRONG,
}

#[repr(C)]
#[derive(Debug)]
pub(crate) enum SyrupType {
    MACADAMIA,
    VANILLA,
    COCONUT,
    CARAMEL,
    CHOCOLATE,
    POPCORN,
}

#[derive(Debug)]
pub(crate) struct Coffee {
    pub(crate) coffee_int: Intensity,
    pub(crate) name: String,
}

#[derive(Debug)]
pub(crate) struct Americano {
    pub(crate) base: Coffee,
    pub(crate) ml_of_water: u32,
}

#[derive(Debug)]
pub(crate) struct Cappuccino {
    pub(crate) base: Coffee,
    pub(crate) ml_of_milk: u32,
}
#[derive(Debug)]
pub(crate) struct SyrupCappuccino {
    pub(crate) base: Cappuccino,
    pub(crate) syrup: SyrupType,
}

#[derive(Debug)]
pub(crate) struct PuSpLatte {
    pub(crate) base: Cappuccino,
    pub(crate) mg_of_pk_spice: u32,
}
