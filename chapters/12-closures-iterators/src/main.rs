fn main() {
    let add_tax = |price: f64| price * 1.08;
    let prices = [10.0, 25.0, 40.0];

    let taxed_prices: Vec<f64> = prices
        .iter()
        .copied()
        .map(add_tax)
        .filter(|price| *price > 20.0)
        .collect();

    println!("taxed items over $20: {taxed_prices:.2?}");
}
