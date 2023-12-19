use std::collections::HashMap;
use std::io;

fn main() {
    let mut details: HashMap<String, Vec<String>> = HashMap::new();

    println!("*** onboarding ***");

    loop {
        let mut choice: String = String::new();

        println!("1. Enter new user data");
        println!("2. Exit");

        println!("enter choice number");
        io::stdin()
            .read_line(&mut choice)
            .expect("couldn't read line");
        let choice: i32 = match choice.trim().parse() {
            Ok(val) => val,
            Err(_) => continue,
        };

        match choice {
            1 => {
                println!("enter details -> ");

                let mut entry: String = String::new();
                io::stdin()
                    .read_line(&mut entry)
                    .expect("couldn't read entry!");

                let entry: String = entry.trim().to_string();
                let words: Vec<&str> = entry.split(" ").collect();

                if words.len() != 4 {
                    println!("wrong format! add entry like this -> \"add [name] to [department]\"");
                    continue;
                }

                let name: &str = match words.get(1) {
                    Some(&str) => str,
                    None => "couldn't read name properly!",
                };
                let department: &str = match words.get(3) {
                    Some(&str) => str,
                    None => "couldn't read department properly!",
                };

                let e: &mut Vec<String> = details
                    .entry(department.to_string().to_lowercase())
                    .or_insert(Vec::new());
                e.push(name.to_string().to_lowercase());
            }
            2 => {
                println!("--*--*--*--*--*-- exit! --*--*--*--*--*--");
                break;
            }
            _ => println!("--- enter valid choice! ---"),
        }
    }
    println!("details: {:?}", details);
}
