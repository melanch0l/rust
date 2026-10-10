#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

fn describe(message: Message) {
    match message {
        Message::Quit => println!("quit"),
        Message::Move { x, y } => println!("move to ({x}, {y})"),
        Message::Write(text) => println!("text: {text}"),
        Message::ChangeColor(red, green, blue) => {
            println!("color: #{red:02x}{green:02x}{blue:02x}");
        }
    }
}

fn main() {
    let nickname: Option<&str> = Some("Ferris");
    println!("nickname: {}", nickname.unwrap_or("anonymous"));

    describe(Message::Move { x: 3, y: -2 });
    describe(Message::Write(String::from("hello")));
    describe(Message::ChangeColor(255, 128, 0));
    describe(Message::Quit);
}
