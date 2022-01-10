fn main() {
    let discard_elements            = vec![3,4];
    let array                       = vec![1,2,3,1,2,10,133,7,4,4,4];
   // let array_with_no_1ns :Vec<i32> = array_diff(array, discard_elements);
   println!("{:?}", array_dff(array, discard_elements));
}
fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    let mut result_vector :Vec<T> = vec![];
    for element_a in a.iter(){
        for element_b in b.iter() {
            if element_a != element_b {

            }
        }
    }
    return result_vector;

}

fn array_dff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    let reuslt_vec = a.into_iter().filter(|x| show_dff(x, b.iter())).map(|x| x).collect();
    return reuslt_vec;
}

fn show_dff<'a, I, T: PartialEq>(x: T, vector_iter: I) -> bool
where
    I: Iterator<Item = T>, {
    for element in vector_iter {
        if element != x {
            continue;
        }
        else {
            return false;
        }
    }
    return true
}

