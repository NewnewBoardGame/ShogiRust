use std::io;

struct AStruct {
    aField: String,
    anotherField: String,
}

trait ATrait {
    fn name(&self) -> String;
}

fn main() {
    println!("Hello, world! From Rust..");

    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();
    let guessPtr = &mut guess;

    io::stdin()
      .read_line(guessPtr)
      .expect("Failed to read line");

    println!("You guessed: {guess}");
}

