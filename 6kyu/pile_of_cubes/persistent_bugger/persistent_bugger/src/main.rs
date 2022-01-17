fn main() {
   let per = persistence(13);
   println!("{}", per);
}
fn persistence(num: u64) -> u64 {
    let mut multiplicative_persistence :u64 = 0;
    let mut new_num                    :u64 = num;
    loop{
        let number_in_string = new_num.to_string();
        if number_in_string.len() == 1 {
            return multiplicative_persistence;
        }
        else {
            new_num = 1;
            multiplicative_persistence +=1;
            for chararcter in number_in_string.chars() {
                new_num = new_num * (chararcter as u64 - 48);
            }
        }
    }
}