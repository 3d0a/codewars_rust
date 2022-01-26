use std::collections::HashMap;
fn main() {
    println!("Hello, world!");
    count_duplicates("AAA1122fff");
}

fn count_duplicates(text: &str) -> u32 {
    let mut dup_counter_map :HashMap<char, u32> = HashMap::new();
    for character in text.to_lowercase().chars() {
        match dup_counter_map.get_mut(&character) {
            Some(count) => {
                *count +=1 ;
            }
            None => {
                dup_counter_map.insert(character, 1);
            }
        }
    }
    let mut max_elem :u32 = 0;
    let max_elem          = dup_counter_map.iter().max_by(|a, b| a.1.cmp(&b.1)).map(|x|  {if x.1 > &max_elem {max_elem = *x.1;} x} ).unwrap();
    let result_count      = dup_counter_map.iter().filter(|x| x.1 == max_elem.1).count();
    
    return result_count as u32;
}