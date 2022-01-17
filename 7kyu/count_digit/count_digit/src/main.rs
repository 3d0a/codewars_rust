fn main() {
    let n = 11;
    let mut squares :Vec<i32>  = (0..n).map(|x| x*x).filter(|x| x.to_string().contains('1')).collect();
    println!("{:?}", nb_dig(25, 1));
}

fn nb_dig(n: i32, d: i32) -> i32 {
    // your code
    let d_as_char          :char          = d.to_string().parse::<char>().unwrap();
    let squares            :Vec<String>   = (0..n+1).map(|x| x*x).filter(|x| x.to_string().contains(d_as_char)).map(|x| x.to_string()).collect();
    let quontity_of_digits :i32           = squares.join("").matches(d_as_char).count() as i32;
    return quontity_of_digits;
}
