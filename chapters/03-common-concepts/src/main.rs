fn main() {
    let temperature_c = 20.0;
    let temperature_f = to_fahrenheit(temperature_c);
    println!("{temperature_c}°C is {temperature_f}°F");

    for number in 1..=5 {
        if number % 2 == 0 {
            println!("{number} is even");
        } else {
            println!("{number} is odd");
        }
    }
}

fn to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}
