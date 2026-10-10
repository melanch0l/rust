use std::collections::HashMap;

fn main() {
    let scores = vec![10, 20, 30];
    let total: i32 = scores.iter().sum();
    println!("scores: {scores:?}; total: {total}");

    let text = String::from("hello") + " Rust";
    println!("text: {text}");

    let mut visits = HashMap::new();
    visits.insert(String::from("library"), 3);
    visits
        .entry(String::from("library"))
        .and_modify(|count| *count += 1);
    visits.entry(String::from("park")).or_insert(1);
    println!("visits: {visits:?}");
}
