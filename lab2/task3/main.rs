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

    let displays = vec![display1, display2, display3.clone()];

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
    let mut assistant = Assistant {
        assigned_diplays: displays,
        assistant_name: "Jhon".to_string(),
    };

    assistant.assign_display(Display::default());
    assistant.assist();
    if let Some(display_bought) = assistant.buy_display(&display3) {
        println!(
            "assistant:{} bought:{display_bought:?}",
            assistant.assistant_name
        );
    }
    if let Some(_) = assistant.buy_display(&display3) {
    } else {
        println!("couldn't find {display3:?}");
    }
}

struct Assistant {
    assistant_name: String,
    assigned_diplays: Vec<Display>,
}
impl Assistant {
    fn assign_display(&mut self, display: Display) {
        self.assigned_diplays.push(display);
    }

    fn assist(&self) {
        let displays = &self.assigned_diplays;
        for i in 0..displays.len() - 1 {
            displays[i].compare_sharpness(&displays[i + 1]);
            displays[i].compare_size(&displays[i + 1]);
            displays[i].compare_with_monitor(&displays[i + 1]);
        }
        displays[displays.len() - 1].compare_sharpness(&displays[0]);
        displays[displays.len() - 1].compare_size(&displays[0]);
        displays[displays.len() - 1].compare_with_monitor(&displays[0]);
    }

    fn buy_display(&mut self, d: &Display) -> Option<Display> {
        let displays = &self.assigned_diplays;
        for i in 0..displays.len() {
            if *d == displays[i] {
                return Some(self.assigned_diplays.swap_remove(i));
            }
        }
        return None;
    }
}

#[derive(PartialEq, Default, Debug, Clone)]
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
