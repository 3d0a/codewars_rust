fn main() {
    println!("{}", order_weight("2000 10003 1234000 44444444 9999 11 11 22 123"));
}

fn order_weight(s: &str) -> String {
    let mut str_to_vec :Vec<&str> = s.split(' ').collect::<Vec<&str>>();
    str_to_vec.sort_by(|a, b| a.cmp(b));
    str_to_vec.sort_by(|a, b| a.chars()
        .map(|x| x.to_string().parse::<i32>().unwrap()).sum::<i32>()
        .cmp(&b.chars().map(|x| x.to_string().parse::<i32>().unwrap()).sum()));
    str_to_vec.join(" ")
}