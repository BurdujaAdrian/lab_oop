use crate::task1::*;
use crate::task2::*;
use std::io;
#[derive(Default)]
enum CoffeeType {
    #[default]
    COFE,
    CAP,
    AMER,
    SYCAP,
    PUMPK,
}

#[derive(Default)]
struct Order {
    c_type: CoffeeType,
    inten: Option<Intensity>,
    q1: u32,
    q2: u32,
    syr: Option<SyrupType>,
}

pub struct Barista {
    queue: Vec<Box<dyn CoffeeTrait>>,
}
fn print_menu() {
    println!(
        "
Menu:

Americano   : 18 mdl
Cappuccino  : 20 mdl
syrup cappuccino     : 25 mdl 
Pumpkin Spice latte : 30 mdl

Avaliable coffee intensities:
    - LIGHT
    - NORMAL
    - STRONG

Available syrups:
    - CARAMEL
    - CHOCOLATE
    - COCONUT
    - MACADOMIA
    - POPCORN
    - VANILLA

"
    );
}
impl Barista {
    pub fn init() -> Self {
        Barista { queue: vec![] }
    }
    pub fn print_queue(&self) {
        for drink in &self.queue {
            println!("printing queue");
            drink.printCoffeeDetails();
            println!();
        }
    }

    pub fn recieve_coffee(&mut self) {
        if let Some(order) = self.brew() {
            println!("You get: ");
            order.printCoffeeDetails();
            return;
        }
        println!("Queue is empty, maybe you should order some coffee !");
    }
    fn brew(&mut self) -> Option<Box<dyn CoffeeTrait>> {
        self.queue.pop()
    }

    pub fn order(&mut self) {
        print_menu();
        let mut pending_order = Order::default();
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("failed to read from stdin");

        let order = buffer.trim().to_lowercase();
        println!("you ordered :{order}\n");
        match order.as_str() {
            "coffee" => {
                pending_order.c_type = CoffeeType::COFE;
                println!("preparing coffee...");
                self.prepare_coffee(&mut pending_order);
                self.queue
                    .push(Box::new(Coffee::makeCoffee(pending_order.inten.unwrap())));
                println!("Order finished, please wait in the queue");
            }
            "cappuccino" => {
                println!("preparing cappuccino...");
                pending_order.c_type = CoffeeType::CAP;
                self.prepare_cappuccino(&mut pending_order);
                self.queue.push(Box::new(Cappuccino::makeCappuccino(
                    pending_order.inten.unwrap(),
                    pending_order.q1,
                )));
                println!("Order finished, please wait in the queue");
            }
            "americano" => {
                println!("preparing americano...");
                pending_order.c_type = CoffeeType::AMER;
                self.prepare_americano(&mut pending_order);
                self.queue.push(Box::new(Americano::makeAmericano(
                    pending_order.inten.unwrap(),
                    pending_order.q1,
                )));
                println!("Order finished, please wait in the queue");
            }
            "cappuccino with syrup" | "syrup cappuccino" => {
                println!("preparing syrup cappuccino...");
                pending_order.c_type = CoffeeType::SYCAP;
                self.prepare_syrup_cappuccino(&mut pending_order);
                self.queue
                    .push(Box::new(SyrupCappuccino::makeSyrupCappuccino(
                        pending_order.inten.unwrap(),
                        pending_order.q1,
                        pending_order.syr.unwrap(),
                    )));
                println!("Order finished, please wait in the queue");
            }
            "pumpkin spice latte" => {
                println!("preparing pumpkin spice latte...");
                pending_order.c_type = CoffeeType::PUMPK;
                self.prepare_pk_spice_latte(&mut pending_order);
                self.queue.push(Box::new(PuSpLatte::makePuSpLatte(
                    pending_order.inten.unwrap(),
                    pending_order.q1,
                    pending_order.q2,
                )));
                println!("Order finished, please wait in the queue");
            }
            _ => {
                println!("{order} isn't on the menu");
            }
        }
    }

    fn prepare_cappuccino(&mut self, pending_order: &mut Order) {
        self.prepare_coffee(pending_order);
        println!("How many ml of milk do you want:");
        println!("1:    50 ml");
        println!("2:   100 ml");
        println!("3:   150 ml");

        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("failed to read from stdin");

        let order = buffer.trim().to_lowercase();

        match order.as_str() {
            "50" | "1" => {
                println!("\nadding 50 ml");
                pending_order.q1 = 50;
            }
            "100" | "2" => {
                println!("\nadding 100 ml");
                pending_order.q1 = 100;
            }
            "150" | "3" => {
                println!("\nadding 150 ml");
                pending_order.q1 = 150;
            }
            _ => return println!("This option is not supported, please try again"),
        }
    }
    fn prepare_coffee(&mut self, pending_order: &mut Order) {
        println!("How intense do you want your coffee to be:");
        println!("1:    LIGHT");
        println!("2:    NORMAL");
        println!("3:    STRONG");
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("failed to read from stdin");

        let order = buffer.trim().to_lowercase();

        match order.as_str() {
            "light" | "1" => {
                println!("\nmaking light coffee");
                pending_order.inten = Some(Intensity::LIGHT);
            }
            "normal" | "2" => {
                println!("\nmaking normal coffee");
                pending_order.inten = Some(Intensity::NORMAL);
            }
            "strong" | "3" => {
                println!("\nmaking strog coffee");
                pending_order.inten = Some(Intensity::STRONG);
            }
            _ => return println!("This option is not supported, please try again"),
        }
    }
    fn prepare_americano(&mut self, pending_order: &mut Order) {
        self.prepare_coffee(pending_order);

        println!("How many ml of water do you want:");
        println!("1:    50 ml");
        println!("2:   100 ml");
        println!("3:   150 ml");

        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("failed to read from stdin");

        let order = buffer.trim().to_lowercase();

        match order.as_str() {
            "50" | "1" => {
                println!("\nadding 50 ml");
                pending_order.q1 = 50;
            }
            "100" | "2" => {
                println!("\nadding 100 ml");
                pending_order.q1 = 100;
            }
            "150" | "3" => {
                println!("\nadding 150 ml");
                pending_order.q1 = 150;
            }
            _ => return println!("This option is not supported, please try again"),
        }
    }
    fn prepare_syrup_cappuccino(&mut self, pending_order: &mut Order) {
        self.prepare_cappuccino(pending_order);

        println!("Which one of these syrups do you want to be added:");
        println!("1:MACADAMIA");
        println!("2:VANILLA");
        println!("3:COCONUT");
        println!("4:CARAMEL");
        println!("5:CHOCOLATE");
        println!("6:POPCORN");
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("failed to read from stdin");

        let order = buffer.trim().to_uppercase();

        match order.as_str() {
            "1" | "MACADAMIA" => {
                println! {"you chose MACADAMIA"};
                pending_order.syr = Some(SyrupType::MACADAMIA);
            }
            "2" | "VANILLA" => {
                println!("you chose VANILLA");
                pending_order.syr = Some(SyrupType::VANILLA);
            }
            "3" | "COCONUT" => {
                println!("you chose COCONUT");
                pending_order.syr = Some(SyrupType::COCONUT);
            }
            "4" | "CARAMEL" => {
                println!("you chose CARAMEL");
                pending_order.syr = Some(SyrupType::CARAMEL);
            }
            "5" | "CHOCOLATE" => {
                println!("you chose CHOCOLATE");
                pending_order.syr = Some(SyrupType::CHOCOLATE);
            }
            "6" | "POPCORN" => {
                println!("you chose POPCORN");
                pending_order.syr = Some(SyrupType::POPCORN);
            }

            _ => return println!("This option is not supported, please try again"),
        }
    }
    fn prepare_pk_spice_latte(&mut self, pending_order: &mut Order) {
        self.prepare_cappuccino(pending_order);
        println!("How many mg of pumpkin spice do you want:");
        println!("1:    50 mg");
        println!("2:   100 mg");
        println!("3:   150 mg");

        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("failed to read from stdin");

        let order = buffer.trim().to_lowercase();

        match order.as_str() {
            "50" | "1" => {
                println!("\nadding 50 mg");
                pending_order.q2 = 50;
            }
            "100" | "2" => {
                println!("\nadding 100 mg");
                pending_order.q2 = 100;
            }
            "150" | "3" => {
                println!("\nadding 150 mg");
                pending_order.q2 = 150;
            }
            _ => return println!("This option is not supported, please try again"),
        }
    }
}

pub fn test() {
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

    let mut barista: Barista = Barista { queue: coffies };

    barista.print_queue();
    barista.brew().unwrap().printCoffeeDetails();
    barista.order();
}
