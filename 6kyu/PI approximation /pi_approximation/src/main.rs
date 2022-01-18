pub const PI: f64 = 3.14159265358979323846264338327950288f64;
fn main() {
    println!("{:?}", iter_pi(0.01));
}

fn rnd10(f: f64) -> f64 { (f * 1e10).round() / 1e10 }

fn iter_pi(epsilon: f64) -> (i32, f64) {
    let mut iteration_number :f64 = 0.;
    let mut pi_approximation :f64 = 0.;
    loop {
        pi_approximation = pi_approximation + 2./((4.*iteration_number + 1.) * (4.*iteration_number + 3.));
        if (rnd10(4.*pi_approximation) - rnd10(PI)).abs() < epsilon {
            return (iteration_number  as i32, rnd10(4.*pi_approximation));
        }
        else {
            iteration_number += 1.; 
        }
    }
}