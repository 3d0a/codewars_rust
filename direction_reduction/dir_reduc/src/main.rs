use std::collections::HashMap;
fn main() {
    let arr = &[Direction::North, Direction::East, Direction::West, Direction::South];
    println!("{:?}", dir_reduc(arr));
}
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq,)]
enum Direction {
    North,
    East,
    West,
    South,
}

fn dir_reduc(arr: &[Direction]) -> Vec<Direction> {
    let mut reduced_array :Vec<Direction> = vec![];
    let mut direction_weights :HashMap<Direction, i32> = HashMap::new();
    direction_weights.insert(Direction::North,  1);
    direction_weights.insert(Direction::South, -1);
    direction_weights.insert(Direction::East,   2);
    direction_weights.insert(Direction::West,  -2);
    for element in arr {
        reduced_array.push(*element);
    }
    let mut _i = 0;
    loop {
        if _i == reduced_array.len()  || _i == reduced_array.len() - 1 {
            break;
        } 
        if direction_weights[&reduced_array[_i]] + direction_weights[&reduced_array[_i+1]] == 0 {
            reduced_array.remove(_i); 
            reduced_array.remove(_i); 
            _i = 0;
            continue;
        }
        _i += 1;
    }

    return reduced_array;
}