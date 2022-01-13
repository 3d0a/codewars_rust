fn main() {
    let n = 11;
    let mut squares :Vec<i32>  = (0..n).map(|x| x*x).filter(|x| x.to_string().contains('1')).collect();
    println!("{}", nb_dig(11, 1));
}

fn nb_dig(n: i32, d: i32) -> i32 {
    // your code
    let d_as_char   :char = d.to_string().parse::<char>().unwrap();
    let mut squares :i32  = (0..n).map(|x| x*x).filter(|x| x.to_string().contains(d_as_char)).count() as i32;
    return squares;
}
