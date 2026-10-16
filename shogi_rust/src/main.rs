use std::io;

struct AStruct {
    a_field: String,
    another_field: String,
}

trait ATrait {
    fn name(&self) -> String;
}

fn main() {
    println!("Hello, world! From Rust..");

    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();
    let guess_ptr = &mut guess;

    io::stdin()
      .read_line(guess_ptr)
      .expect("Failed to read line");

    println!("You guessed: {guess}");

    println!("Hello, world!");
    let foo = 25;
    println!("{}", foo);

    writeToDatabase();
}

fn writeToDatabase() {
    println!("Placeholder...")
}
