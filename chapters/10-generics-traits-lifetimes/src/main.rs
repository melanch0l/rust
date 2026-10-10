use std::fmt::Display;

fn largest<T: PartialOrd>(items: &[T]) -> Option<&T> {
    let mut largest = items.first()?;
    for item in &items[1..] {
        if item > largest {
            largest = item;
        }
    }
    Some(largest)
}

trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    title: String,
    author: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

fn announce(item: &impl Summary, extra: &impl Display) {
    println!("Breaking: {} ({extra})", item.summarize());
}

fn main() {
    let numbers = [34, 50, 25, 100, 65];
    println!("largest: {:?}", largest(&numbers));

    let article = Article {
        title: String::from("Rust makes ownership explicit"),
        author: String::from("Alex"),
    };
    announce(&article, &2026);
}
