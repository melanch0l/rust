mod restaurant {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Added to the waitlist.");
        }
    }

    pub fn open() {
        println!("The restaurant is open.");
    }
}

fn main() {
    restaurant::open();
    restaurant::hosting::add_to_waitlist();
}
