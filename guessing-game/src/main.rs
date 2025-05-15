// Adapted from the great 'Idiomatic Rust' book
// https://fios-quest.com/idiomatic-rust-in-simple-steps/language-basics/memory.html

fn main() {
    print!("Welcome to THE GUESSING GAME! ");
    println!("The house will pick one of three options:");
    println!("1. Red door");
    println!("2. Blue door");
    println!("3. Green door");
    println!("There might be a reward waiting for you behind one of them... Good luck!");

    println!("Which door do you walk through?");
    let input = std::io::stdin()
        .lines()
        .next()
        .expect("Your input was empty!")
        .expect("There was an error reading the input!");

    let colors = ["Red", "Green", "Blue"];
    let time = std::time::UNIX_EPOCH
        .elapsed()
        .expect("Time broke!")
        .as_millis() as usize; // We only need the LSB, so it's okay to overflow here

    let actual = colors[time % colors.len()];
    println!("The color I chose was {actual}. You chose {input}.");
    
    if input == actual {
        println!("You win!");
    } else {
        println!("You lose...");
    }
}
