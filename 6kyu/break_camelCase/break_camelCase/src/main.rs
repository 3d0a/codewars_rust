fn main() {
    println!("{:?}",solution("HelloWorld!Sir"));
}

fn solution(s: &str) -> String {
    let mut final_string :String = String::new();
    let alphabet                 = (b'A'..=b'Z').map(|c| c as char)
                                                .filter(|c| c.is_alphabetic())
                                                .collect::<Vec<_>>();

    for character in s.chars() {
        if alphabet.contains(&character) {
            final_string.push(' ');
            final_string.push(character);
        }
        else {
            final_string.push(character);
        }
    }
    return final_string.trim().to_owned();
}