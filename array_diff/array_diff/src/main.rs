fn main() {
    let discard_elements = &[3,4];
    let array = [1,2,3,1,2,10,133,7,4,4,4];
    let array_with_no_1ns :Vec<i32> = array.into_iter().filter(|x|  not_in_array(x, discard_elements)).map(|x| x).collect();
    println!("{:?}", array_with_no_1ns);
}

fn not_in_array(number :&i32, array :&[i32]) -> bool {
    for element in array {
        if number != element{
           continue;
        }
        else{
            return false;
        }
    }
    return true;
}