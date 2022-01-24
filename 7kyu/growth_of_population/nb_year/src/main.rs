fn main() {
    println!("{:?}", nb_year(1500, 5., 100, 5000));
}

fn nb_year(p0: i32, percent: f64, aug: i32, p: i32)-> i32 {
    let mut inhabitants    = p0;
    let mut years          = 0;
    loop {
        if inhabitants >= p {
            return years
        }
        inhabitants = inhabitants + aug + (inhabitants as f64 * percent/100.) as i32;
        years += 1;
    }
}