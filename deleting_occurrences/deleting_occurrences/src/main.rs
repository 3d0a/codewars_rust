fn main() {
    let result_vector = delete_nth(&[1,1,3,3,7,2,2,2,2], 3);
    println!("{:?}", result_vector);
}

fn delete_nth(lst: &[u8], n: usize) -> Vec<u8> {
    use std::collections::HashMap;
    let mut value_repeat_counter_dict :HashMap<&u8, usize> = HashMap::new();
    let mut pictures_vector           :Vec<u8>             = vec![];

    for element in lst.iter(){
        match value_repeat_counter_dict.get(element){
            Some(quontity) => {
                let new_quontity = quontity + 1;
                if new_quontity <= n {
                    value_repeat_counter_dict.remove(element);
                    value_repeat_counter_dict.insert(element, new_quontity);
                    pictures_vector.push(element+0);    
                }
                else{
                    continue;
                }
            }
            None => {
                value_repeat_counter_dict.insert(element, 1);
                pictures_vector.push(element+0);   
            }
        }
    }
    return pictures_vector;
}