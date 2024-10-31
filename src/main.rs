mod task1;
mod task2;
mod task3;
use task1::*;
use task2::*;

fn main() {
    // make each coffee
    let coffies: Vec<Box<dyn CoffeeTrait>> = vec![
        Box::new(Coffee::makeCoffee(Intensity::NORMAL)),
        Box::new(Americano::makeAmericano(Intensity::STRONG, 50)),
        Box::new(Cappuccino::makeCappuccino(Intensity::NORMAL, 100)),
        Box::new(SyrupCappuccino::makeSyrupCappuccino(
            Intensity::NORMAL,
            100,
            SyrupType::COCONUT,
        )),
        Box::new(PuSpLatte::makePuSpLatte(Intensity::LIGHT, 50, 50)),
    ];

    for drink in coffies {
        drink.printCoffeeDetails();
        println!();
    }
}
