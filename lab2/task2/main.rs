#![allow(unreachable_code, dead_code)]
use std::env;
use std::fs;

fn main() {
    let mut files: Vec<TextData> = vec![];
    for path in env::args().skip(1) {
        match fs::read_to_string(&path) // readFileIntoString(String path)
            // I assume I don't have to wrap this function
            // into a different struct for no reason
        {
            Ok(contents) => files.push(initTextData(path,contents)),
            Err(err) => println!("file {path} reading error: {:?}\n", err),
        }
    }
    //println!("succcesfully read data:{files:#?}");

    return;

    // for later
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

#[derive(Default)]
struct TextData {
    file_name: String,
    text: String,
    n_of_vowels: Option<u32>,
    n_of_consonants: Option<u32>,
    n_of_letters: Option<u32>,
}
const CONSONANTS: &str = "qwrtpsdfghjklzxcvbnm";
impl TextData {
    fn get_numb_of_vowels(&mut self) -> u32 {
        if let Some(number) = self.n_of_vowels {
            return number;
        }

        let mut number = 0;
        for char in self.text.to_lowercase().chars() {
            match char {
                'o' | 'a' | 'i' | 'e' | 'u' => number += 1,
                _ => (),
            }
        }
        self.n_of_vowels = Some(number);
        return number;
    }

    fn get_number_of_consonants(&mut self) -> u32 {
        if let Some(number) = self.n_of_consonants {
            return number;
        }
        let mut number = 0;
        self.text.to_lowercase().chars().for_each(|char| {
            if char.is_alphabetic() {
                if CONSONANTS.contains(char) {
                    number += 1;
                }
            }
        });
        self.n_of_consonants = Some(number);
        return number;
    }

    fn get_number_of_letters(&mut self) -> u32 {
        if let Some(number) = self.n_of_letters {
            return number;
        }
        let mut number = 0;

        // if we have them already calculated, might as well
        if let (Some(vowels), Some(consns)) = (self.n_of_vowels, self.n_of_consonants) {
            number = vowels + consns;
            self.n_of_letters = Some(number);
            return number;
        }
        for char in self.text.chars() {
            if char.is_alphabetic() {
                number += 1;
            }
        }
        self.n_of_letters = Some(number);
        return number;
    }
}

#[allow(non_snake_case)]
fn initTextData(path: String, contents: String) -> TextData {
    TextData {
        file_name: path,
        text: contents,
        ..TextData::default() // Use the default values for the other fields
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
