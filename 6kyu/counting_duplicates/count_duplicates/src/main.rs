use std::collections::HashMap;
fn main() {
    println!("Hello, world!");
    println!("{}", count_duplicates("kue3Cx3RMEd0qNZacqCe6N"));
}

fn count_duplicates(text: &str) -> u32 {
    if text == "" {
        return 0;
    }
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
    let result_count      = dup_counter_map.iter().filter(|x| x.1 > &1).count();
    println!("{:?} {}",dup_counter_map, result_count );
    return result_count as u32;
}