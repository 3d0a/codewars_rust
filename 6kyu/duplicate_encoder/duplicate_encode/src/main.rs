fn main() {
    println!("{}",duplicate_encode("CodeWarrior"));
}

fn duplicate_encode(word: &str) -> String {
    let mut result_string :String = String::new();
    let mut word_as_arr = word.chars().collect::<Vec<char>>();
    print!("{}", word);
    if word_as_arr[0].is_uppercase() {
        word_as_arr[0] = word_as_arr[0].to_lowercase().collect::<Vec<char>>()[0];
    }
    for character in word_as_arr.iter() {
        if word_as_arr.iter().filter(|x| *x == character).count() == 1 {
            result_string.push('(');
        }
        else {
            result_string.push(')');
        }
    }
    result_string
}