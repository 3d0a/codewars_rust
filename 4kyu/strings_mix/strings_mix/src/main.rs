use std::collections::HashMap;
fn main() {
    println!("Hello, world!");
    let s1 = "my&friend&Paul has heavy hats! &";
    let s2 = "my friend John has many many friends &";
    mix(s1, s2);
}
fn mix(s1: &str, s2: &str) -> String {
    let return_string :String = String::new();
    let alphabet = (b'a'..=b'z')
        .map(|c| c as char)
        .filter(|c| c.is_alphabetic())
        .collect::<Vec<_>>(); 
    let mut first_string_vec  :Vec<String> = vec_of_lowercase(s1, &alphabet);
    let mut second_string_vec :Vec<String> = vec_of_lowercase(s2, &alphabet);
    println!("{:?}, {:?}", char_counter(&first_string_vec),char_counter(&second_string_vec));

    return return_string;
  }

fn vec_of_lowercase(string_to_convert :&str, alphabet :&Vec<char>) -> Vec<String> {
    return string_to_convert.chars()
        .filter(|x| alphabet.contains(x))
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
}

fn char_counter(strings_vec :&Vec<String>) -> HashMap<&String, u64> {
    let mut counts_map :HashMap<&String, u64> = HashMap::from([(&strings_vec[0], 0)]);
    for character in strings_vec {
        match counts_map.get_mut(character) {
            Some(value) => {
                *value +=1;
            }
            None => {
                counts_map.insert(character, 1);
            }
        }
    }
    return counts_map;
}


