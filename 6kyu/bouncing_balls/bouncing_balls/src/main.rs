fn main() {
    let result = bouncing_ball(3.0, 0.66, 1.5);
    println!("{}", result);
}

fn bouncing_ball(h: f64,  bounce: f64,  window: f64) -> i32 {
    let mut bounce_counter    : i32 =   0;
    let mut direction         : i32 =  -1;
    let mut decreased_height        =   h ;
    loop {
        if h <= 0.0  {
            println!(" Float parameter 'h' in meters must be greater than 0");
            break;
        }
        else if window >= h {
            println!("Float parameter 'window' must be less than h");
            break;
        }
        else if bounce >= 1.0 || bounce < 0.0 {
            println!(" Float parameter 'bounce' must be greater than 0 and less than 1");
            break;
        }
        else {
            if direction < 0 && decreased_height > window  {
                bounce_counter = bounce_counter + 1;  
                decreased_height = decreased_height * bounce; 
            }
            else if direction > 0 && decreased_height > window {
                bounce_counter = bounce_counter + 1;
            }
            else {
                return bounce_counter;
            }
            direction = direction * -1;

        }
    }
    return -1;
}
