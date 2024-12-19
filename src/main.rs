use std::io;

struct Option {
    number: u32,
    description: String
}

impl Option {
    fn new(number: u32, description: &str) -> Self {
        Self {
            number,
            description: description.to_string()
        }
    }
}

struct Champion {
    name: String,
    health: u32,
    attack: u32,
}

impl Champion {
    fn new(name: &str, health: u32, attack: u32) -> Self {
        Self {
            health,
            attack,
            name: name.to_string()
        }
    }
}

fn main() {
    let _my_option = Option::new(1, "This is an example description");
    println!("Welcome to Underworld Legends!");

    println!("Please enter your name .");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line.");



    let champion = Champion::new(&name, 100, 10);
    println!("Welcome {}", champion.name);

    let intro = r#"
    You wake up dazed and confused. Unsure of how you got here, you
    begin feeling your surroundings. The air is cold and damp -- you slowly work
    up the strength to stand up.

    The sound of foot steps echo in the distance.

    What say you?";
    "#;

    println!("{}",intro);

    let options: [&str; 3] = ["Option A", "Option B", "Option C"];

    for &option in &options {
        println!("{}", &option);
    }

    let mut user_choice = String::new();

    io::stdin()
        .read_line(&mut user_choice)
        .expect("Failed to read option.");

    if options.contains(&user_choice.trim()) {
        println!("User chose a valid option.")
    } else {
        println!("User did not choose a valid option.")
    }

}

