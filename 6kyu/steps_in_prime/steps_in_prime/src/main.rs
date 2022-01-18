fn main() {
    println!("{:?}", step(8, 30000,100000));
}
fn step(g: i32, m: u64, n: u64) -> Option<(u64, u64)> {
    let vec_of_primes :Vec<u64> = (m..=n).into_iter().filter(|x| is_prime(*x) ).collect::<Vec<u64>>();
    let mut _i = 0;
    while _i < vec_of_primes.len() - 1 {
        if vec_of_primes.contains(&((g + vec_of_primes[_i] as i32)as u64)) {
            return Some((vec_of_primes[_i], (vec_of_primes[_i]) + g as u64));
        } 
        _i +=1;
    }
    return None;
}

fn is_prime(number :u64) -> bool {
    if number <= 3 {
        return number > 1;
    }
    if number % 2 == 0 || number % 3 == 0 {
        return false;
    }
    let mut _i :u64 = 5;
    while _i.pow(2) <= number {
        if number % _i == 0 || number % (_i + 2) == 0 {
            return false;
        }
        _i+=6;
    }
    return true;
}
