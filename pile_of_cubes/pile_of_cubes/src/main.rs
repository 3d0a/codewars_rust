fn main() {
    println!("{}", u64::MAX);
    println!("{}",find_nb(91716553919377));
}

fn find_nb(m: u64) -> i32 {
    let mut volume         :u64 = 0;
    let mut current_count  :i32 = 1;
    loop {
        match current_count.checked_mul(current_count.pow(3)) {
            Some(_v) => {
                volume = volume + current_count.pow(3) as u64;
            }
            None => {
                break;
            }
        }
        if volume == m {
            return current_count;
        }
        else if volume > m {
            break;
        }
        current_count +=1;  
    }    
    return -1;
}