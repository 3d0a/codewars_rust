fn main() {
    println!("Hello, world!");
}

fn feast(beast: &str, dish: &str) -> bool {
    if beast.chars().next() == dish.chars().next() && beast.chars().last().unwrap() == dish.chars().last().unwrap() {
        return true;
    }
    return false;
}