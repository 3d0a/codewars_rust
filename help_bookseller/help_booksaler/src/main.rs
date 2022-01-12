use std::collections::HashMap;

fn main() {
    let mut b = vec!["BBAR 150", "CDXE 515", "BKWR 250", "BTSQ 890", "DRTY 600"];
    let mut c = vec!["A", "B", "C", "D"];
    let hell = "hello world";
    println!("{}", stock_list(b,c));
    
}

fn stock_list(list_art: Vec<&str>, list_cat: Vec<&str>) -> String {
    let mut stock_list_hashmap :HashMap<char, u32> = HashMap::new();
    let mut stock_list_string                      = String::new(); 

    for element in list_art {
        let element_as_vec         :Vec<&str> = element.split_whitespace().collect();
        let mut first_elem         :char      = element_as_vec[0].chars().next().unwrap();
        let book_type_quintity     :u32       = element_as_vec[1].parse().unwrap();

        match stock_list_hashmap.get(&first_elem) {
            Some(mut quontity) => {
                let new_quontity :u32 = element_as_vec[1].parse().unwrap();
                quontity = &(quontity + new_quontity);
            }
            None => {
                let new_quontity :u32 = element_as_vec[1].parse().unwrap();
                stock_list_hashmap.insert(first_elem, new_quontity);
            }
        }
    }
    
    for (key, value) in stock_list_hashmap {
        stock_list_string.push('(');
        stock_list_string.push(key);
        stock_list_string.push('-');
        let number_to_str :&str = &value.to_string();
        stock_list_string.push_str(number_to_str);
        stock_list_string.push(')');
        stock_list_string.push(' ');
    }
    return stock_list_string;
}
