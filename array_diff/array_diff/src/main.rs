fn main() {
    let discard_elements            = vec![3,4];
    let array                       = vec![1,2,3,1,2,10,133,7,4,4,4];
   // let array_with_no_1ns :Vec<i32> = array_diff(array, discard_elements);
   println!("{:?}", array_diff(array, discard_elements));
}
fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    let reuslt_vec = a.into_iter().filter(|x| ! b.contains(x)).map(|x| x).collect();
    return reuslt_vec;
}


