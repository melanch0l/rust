#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_contain(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let room = Rectangle {
        width: 30,
        height: 50,
    };
    let desk = Rectangle {
        width: 10,
        height: 20,
    };

    println!("{room:?} has area {}.", room.area());
    println!("Can the room contain the desk? {}", room.can_contain(&desk));
}
