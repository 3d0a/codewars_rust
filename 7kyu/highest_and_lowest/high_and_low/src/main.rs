fn main() {
    println!("{}", high_and_low("1 2 3 4 5"));
}

fn high_and_low(numbers: &str) -> String {
    let min_element :String = numbers.split(' ').map(|c| c.parse::<i32>().unwrap()).min().unwrap().to_string();
    let max_element :String = numbers.split(' ').map(|c| c.parse::<i32>().unwrap()).max().unwrap().to_string();
    let two_elements = [max_element, min_element];
    return two_elements.join(" ");
}
