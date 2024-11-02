#![allow(dead_code)]
fn main() {
    let display1 = Display {
        width: 1920,
        height: 1080,
        ppi: 401.0,
        model: String::from("Display 1"),
    };

    let display2 = Display {
        width: 2560,
        height: 1440,
        ppi: 500.0,
        model: String::from("Display 2"),
    };

    let display3 = Display {
        width: 1920,
        height: 1080,
        ppi: 300.0,
        model: String::from("Display 3"),
    };

    let displays = vec![display1, display2, display3];

    for dis1 in &displays {
        for dis2 in &displays {
            if *dis1 == *dis2 {
                continue;
            }
            dis1.compare_sharpness(&dis2);
            dis1.compare_size(&dis2);
            dis1.compare_with_monitor(&dis2);
        }
    }
}
#[derive(PartialEq)]
struct Display {
    width: i32,
    height: i32,
    ppi: f32,
    model: String,
}

impl Display {
    fn compare_size(&self, m: &Display) {
        println!("\n## Comparing sizes:\n");
        if self.width > m.width {
            println!("{} is wider then {}\n", self.model, m.model);
        } else if m.width > self.width {
            println!("{} is wider then {}\n", m.model, self.model);
        } else {
            println!("{} has the same width as {}\n", m.model, self.model);
        }

        if self.height > m.height {
            println!("{} is taller then {}\n", self.model, m.model);
        } else if m.height > self.height {
            println!("{} is taller then {}\n", m.model, self.model);
        } else {
            println!("{} has the same height as {}\n", m.model, self.model);
        }
    }

    fn compare_sharpness(&self, m: &Display) {
        println!("\n## Comparing sharpness:\n");
        if self.ppi > m.ppi {
            println!("{} is sharper then {}\n", self.model, m.model);
        } else if m.ppi > self.ppi {
            println!("{} is sharper then {}\n", m.model, self.model);
        } else {
            println!("{} has the same ppi as {}\n", m.model, self.model);
        }
    }

    fn compare_with_monitor(&self, m: &Display) {
        println!("\n# Comparing both properities:");
        self.compare_size(m);
        self.compare_sharpness(m);
    }
}
