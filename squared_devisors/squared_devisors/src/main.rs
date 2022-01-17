use std::time::{Instant};
fn main() {
    let start_time = Instant::now();
    println!("{:?}, {:?}", list_squared(1, 250), start_time.elapsed());
}
fn list_squared(m: u64, n: u64) -> Vec<(u64, u64)> {
    let mut squared_arr :Vec<(u64, u64)> = vec![];
    for _i in m..=n {
        let mut simple_arr :Vec<u64> = vec![];
        for _j in  1..=((_i as f64).sqrt().round() ) as u64  {
            if _i % _j == 0 {
                simple_arr.push(_j);
                if _j != _i/_j {
                    simple_arr.push(_i/_j);
                }
            }
        }
        let squared_sum :u64 = simple_arr.iter().map(|x| x.pow(2)).sum();
        if (squared_sum as f64).sqrt() % 1. == 0. {
            squared_arr.push((_i, squared_sum));
        }
 
    }
    return squared_arr;
}
