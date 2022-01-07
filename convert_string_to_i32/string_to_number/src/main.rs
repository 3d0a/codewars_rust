

fn main() {
    let number_in_string: &str = "-1337";
    let my_number = string_to_number(number_in_string);
    println!("{}", my_number);
}

fn string_to_number(s: &str) -> i32 {
    let mut char_in_int32  = 0;
    let mut degree         = s.len() - 1;
    let mut sign           = 1;
    println!("{}", degree);
    for character in s.chars() {
        if character == '-' {
            sign = -1;
            degree = degree - 1;
            continue;
        }
        let character     = character as u32;
        char_in_int32     = char_in_int32 + (character as u32 - 48) * u32::pow(10, degree as u32);
        if degree == 0 {
            return char_in_int32 as i32 * sign;  
        }
        degree = degree - 1;
        
    }
    return 1;
}