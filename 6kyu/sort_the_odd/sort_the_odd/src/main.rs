fn main() {
    let array_to_be_sorted = &[9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
    println!("{:?}",sort_array(array_to_be_sorted) );
}
fn sort_array(arr: &[i32]) -> Vec<i32> {
    let mut sorted_vec :Vec<i32> = arr.iter().filter(|x| *x % 2 as i32 != 0).map(|x| *x).collect();
    let mut result_vec :Vec<i32> = vec![];
    sorted_vec.sort();
    sorted_vec.reverse();
    for elem in arr {
        if elem % 2 == 0{
            result_vec.push(*elem);   
        }
        else {
            result_vec.push(sorted_vec.pop().unwrap());
        }
    }
    
    return result_vec;
}