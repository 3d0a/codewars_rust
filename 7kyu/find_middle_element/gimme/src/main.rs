fn main() {
    let arr = [55, 14,3];
    println!( "{}" ,gimme(arr));
}
fn gimme(input_array: [i32;3]) -> usize {
    let mut middle_element = input_array;
    middle_element.sort();
    return input_array.iter().position( |x| *x == middle_element[1]).unwrap();
  }