use std::io;

fn main() {
    let secret = 42;
    println!("Guess the number (1-100):");

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read input");

        let guess: u32 = match input.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Please enter a whole number.");
                continue;
            }
        };

        match guess.cmp(&secret) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You got it!");
                break;
            }
        }
    }
}
