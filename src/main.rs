use std::io;

fn main() {
    println!("Welcome to Underworld Legends!");

    println!("Please enter your name .");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    println!("Welcome {}", name);

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

}

