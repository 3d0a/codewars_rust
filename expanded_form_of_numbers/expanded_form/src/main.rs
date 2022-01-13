fn main() {
    println!("{}", expanded_form(74402));
}

fn expanded_form(n: u64) -> String {
    let mut vector_of_digits :Vec<u64> = n.to_string().chars().map(|x| x as u64 - 48).collect();
    let mut starting_pow     :u32 = (vector_of_digits.len() - 1) as u32;
    let base                 :u64 = 10;
    let string_vec           :Vec<String> = vector_of_digits.into_iter()
                                                       .map(|mut x| {x = x*base.pow(starting_pow); if starting_pow !=0 {starting_pow-=1}; x})
                                                       .filter(|x| *x != 0)
                                                       .map(|x| x.to_string())
                                                       .collect();
    let result_string = string_vec.join(" + ");
    return result_string;
} 