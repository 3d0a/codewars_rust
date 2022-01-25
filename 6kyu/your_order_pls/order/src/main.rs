fn main() {
    println!("{}",order("4of Fo1r pe6ople g3ood th5e the2"));
}

fn order(sentence: &str) -> String {
    if sentence == "" {
        return "".to_owned();
    }
    let split_of_arr         :Vec<&str>        = sentence.split(' ').collect::<Vec<&str>>();
    let mut array_of_tupples :Vec<(i32, &str)> = vec![];
    for word in split_of_arr {
        let number = word.chars().filter(|c| c.is_numeric()).next().unwrap().to_string().parse::<i32>().unwrap();
        array_of_tupples.push((number, word));
    }
    array_of_tupples.sort_by(|a, b| a.0.cmp(&b.0));
    array_of_tupples.into_iter().map(|x| {x.1}).collect::<Vec<&str>>().join(" ")   
}