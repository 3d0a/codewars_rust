fn main() {
    println!("{:?}",solution("HelloWorld!Sir"));
}

fn solution(s: &str) -> String {
    let mut final_string :String = String::new();
    for character in s.chars() {
        if character.is_uppercase() {
            final_string.push(' ');
        }
            final_string.push(character);
    }
    return final_string.trim().to_owned();
}