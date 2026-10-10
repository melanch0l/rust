struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 3, y: 7 };
    let Point { x, y } = point;
    println!("point at ({x}, {y})");

    let config = Some(5);
    if let Some(value) = config {
        println!("configured value: {value}");
    }

    let status = 200;
    match status {
        200..=299 => println!("success"),
        400..=499 => println!("client error"),
        500..=599 => println!("server error"),
        _ => println!("other status"),
    }
}
