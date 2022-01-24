fn main() {
    println!("{:?}", digitize(880055532));
}

fn digitize(n: u64) -> Vec<u8> {
    let mut revese_num_vec :Vec<u8> = n.to_string().chars().map(|c| c.to_digit(10).unwrap() as u8).rev().collect::<Vec<u8>>();
    return revese_num_vec;
    
}