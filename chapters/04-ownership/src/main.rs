fn main() {
    let mut message = String::from("hello");
    append_world(&mut message);
    println!("{message}");

    let first_word = first_word(&message);
    println!("first word: {first_word}");
}

fn append_world(text: &mut String) {
    text.push_str(", world");
}

fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}
