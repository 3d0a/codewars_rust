fn main() {
    println!("{}", printer_errors("aaaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyz"));
}

fn printer_errors(s: &str) -> String {
    let alphabet = (b'a'..=b'm')
        .map(|c| c as char)
        .filter(|c| c.is_alphabetic())
        .collect::<Vec<_>>();
    
    let number_of_errors = s.chars().filter(|c| !alphabet.contains(c)).count();
    format!("{}/{}", number_of_errors, s.len())
}