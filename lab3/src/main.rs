mod task1;
mod task2;
mod task3;
mod task4;

use std::io;
use task4::Barista;
fn main() {
    println!("Welcome to digital barista !");

    loop {
        println!("Here are your options:");
        println!("");
        println!("1. Order a coffee");
        println!("2. Display the queue");
        println!("3. Recieve the next coffee in queue");
        println!("4. Exit");
        println!("");
        print!("Your choose: ");
        let mut barrista: Barista = Barista::init();

        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("failed to read from stdin");

        let order = buffer.trim().to_lowercase();

        match order.as_str() {
            "1" => {
                barrista.order();
            }
            "2" => {
                barrista.print_queue();
            }
            "3" => {
                barrista.recieve_coffee();
            }
            "4" => {
                return;
            }

            _ => return println!("This option is not supported, please try again"),
        }
    }
}
