use regex::Regex;
fn main() {
    println!("Hello, world!");
}

fn validate_pin(pin: &str) -> bool {
    let re = Regex::new(r"^(\d{4}|\d{6})$").unwrap();
    re.is_match(pin)
}