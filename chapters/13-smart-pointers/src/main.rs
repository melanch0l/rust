use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let owned_value = Box::new(5);
    println!("boxed value: {owned_value}");

    let shared_count = Rc::new(RefCell::new(0));
    let first_reference = Rc::clone(&shared_count);
    let second_reference = Rc::clone(&shared_count);

    *first_reference.borrow_mut() += 1;
    *second_reference.borrow_mut() += 1;
    println!("shared count: {}", shared_count.borrow());
}
