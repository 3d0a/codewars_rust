fn main() {
    println!("{}", expanded_form(7442));
}

fn expanded_form(n: u64) -> String {
    let mut vector_of_digits :Vec<i32> = n.to_string().chars().map(|x| x as i32 - 48).collect();
    let mut starting_pow     :i32 = (vector_of_digits.len() - 1) as i32;
    let base                 :i32 = 10;
    let string_vec           :Vec<String> = vector_of_digits.into_iter()
                                                       .map(|mut x| {x = x*base.pow(starting_pow as u32);starting_pow = starting_pow -1; x.to_string()})
                                                       .collect();
    let result_string = string_vec.join(" + ");
    return result_string;
} 