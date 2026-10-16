use std::io;

#[derive(Debug)]
struct AStruct {
    a_field: String,
    another_field: String,
}

trait ATrait {
    fn name(&self) -> String;
}

impl ATrait for AStruct {
    fn name(&self) -> String {
        "test".to_string()
    }
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

    let bar = AStruct{ a_field: "foo".to_string(), another_field: "bar".to_string() };
    println!("bar:{:?}", bar);
    println!("name: {}", bar.name());

    write_to_database();
}

fn write_to_database() {
    println!("Placeholder...")
}
