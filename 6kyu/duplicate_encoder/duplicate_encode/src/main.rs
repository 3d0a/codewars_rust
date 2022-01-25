fn main() {
    println!("Hello, world!");
}

fn duplicate_encode(word: &str) -> String {
    let mut result_string :String = String::new();
    for character in word.to_lowercase().chars() {
        if word.chars().filter(|x| x == &character).count() == 1 {
            result_string.push('(')
        }
        else {
            result_string.push(')')
        }
    }
    result_string
}