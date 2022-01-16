fn main() {
    let number :f64 = 4.;
    println!("{:?}", list_squared(1, 250));
}
fn list_squared(m: u64, n: u64) -> Vec<(u64, u64)> {
    let mut squared_arr :Vec<(u64, u64)> = vec![];
    for _i in (m..=n) {
        let squared_sum :u64 = (1..=_i).filter(|x| _i%x==0).map(|x| x.pow(2)).sum();
        if (squared_sum as f64).sqrt()%1. == 0. {
            squared_arr.push((_i, squared_sum));
        }
    }

    return squared_arr;
}
