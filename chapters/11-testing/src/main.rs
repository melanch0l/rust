fn add(left: i32, right: i32) -> i32 {
    left + right
}

fn main() {
    println!("2 + 3 = {}", add(2, 3));
}

#[cfg(test)]
mod tests {
    use super::add;

    #[test]
    fn adds_two_numbers() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn addition_is_commutative_for_these_values() {
        assert_eq!(add(7, 4), add(4, 7));
    }
}
