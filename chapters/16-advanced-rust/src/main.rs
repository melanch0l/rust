use std::fmt;

struct Meters(u32);
type Kilometers = u32;

trait Printable {
    fn print(&self);
}

impl Printable for Meters {
    fn print(&self) {
        println!("{} meters", self.0);
    }
}

impl fmt::Display for Meters {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} m", self.0)
    }
}

fn print_item(item: &dyn Printable) {
    item.print();
}

fn main() {
    let distance = Meters(1_500);
    let route_length: Kilometers = 4;

    print_item(&distance);
    println!("distance: {distance}; route length: {route_length} km");
}
