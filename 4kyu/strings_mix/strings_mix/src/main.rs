use std::collections::HashMap;
fn main() {
    let s1 = "Are they here";
    let s2 = "yes, they are here";
    println!("{}", mix(s1, s2));
}
fn mix(s1: &str, s2: &str) -> String {
    let mut return_string :String = String::new();
    let alphabet = (b'a'..=b'z')
        .map(|c| c as char)
        .filter(|c| c.is_alphabetic())
        .collect::<Vec<_>>(); 
    let mut first_string_vec   :Vec<String>                 = vec_of_lowercase(s1, &alphabet);
    let mut second_string_vec  :Vec<String>                 = vec_of_lowercase(s2, &alphabet);
    let first_string_map       :HashMap<&String, u64>       = char_counter(&first_string_vec);
    let second_string_map      :HashMap<&String, u64>       = char_counter(&second_string_vec);
    let mut vec_of_tuppls      :Vec<(&String, u64, u64)>    = vec![];
    println!("{:?}, {:?}", first_string_map, second_string_map);
    for element in first_string_map {
        match second_string_map.get(element.0) {
            Some(second_str_count) => {
                if second_str_count > &element.1 {
                    vec_of_tuppls.push((element.0, *second_str_count, 2)); 
                }
                else if second_str_count < &element.1 {
                    vec_of_tuppls.push((element.0, element.1, 1));
                }
                else {
                    vec_of_tuppls.push((element.0, element.1, 3));
                }
            }
            None => {
                continue;
            }
        }
    }
    vec_of_tuppls.sort_by(|a, b| a.0.cmp(&b.0));
    vec_of_tuppls.sort_by(|a, b| a.2.cmp(&b.2));
    vec_of_tuppls.sort_by(|a, b| b.1.cmp(&a.1));
    println!("{:?}", vec_of_tuppls);
    for (character, number, arr_num) in &vec_of_tuppls {
        if number == &1 {
            continue;
        }
        if arr_num == &3 {
            return_string.push_str("=");
        }
        else {
            return_string.push_str(&arr_num.to_string().to_owned());
        }
        return_string.push_str(":");
        return_string.push_str(&character.repeat(*number as usize).to_owned());
        if vec_of_tuppls.last().unwrap().0 == *character {
            println!("{}",character);
            break;
        }
        else {
            return_string.push_str("/");
        }
    }
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


