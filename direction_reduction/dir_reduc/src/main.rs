use std::fmt;
fn main() {
    let arr = &[Direction::North, Direction::North, Direction::North, Direction::South, Direction::West];
    println!("{:?}", dir_reduc(arr));
}

enum Direction {
    North,
    East,
    West,
    South,
}

impl fmt::Debug for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Direction::North => write!(f, "North"),
            Direction::East  => write!(f,  "East"),
            Direction::West  => write!(f,  "West"),
            Direction::South => write!(f, "South"),
        }
    }
}
struct Coordinats {
    x :i32,
    y :i32,
}

fn dir_reduc(arr: &[Direction]) -> Vec<Direction> {
    let mut reduced_array :Vec<Direction> = vec![];
    let mut coordinate    :Coordinats = Coordinats {
        x: 0,
        y: 0
    };

    for direction in arr {
        match direction {
            Direction::North => { coordinate.y = coordinate.y + 1 },
            Direction::East  => { coordinate.x = coordinate.x + 1 },
            Direction::West  => { coordinate.x = coordinate.x - 1 },
            Direction::South => { coordinate.y = coordinate.y - 1 }
        } 
    }

    if coordinate.x > 0 {
        for i in (0..coordinate.x) {
            reduced_array.push(Direction::East);
        }
    }
    else if coordinate.x < 0 {
        for i in (0..coordinate.x * -1) {
            reduced_array.push(Direction::West);
        }       
    }
    else {
    }

    if coordinate.y > 0 {
        for i in (0..coordinate.y) {
            reduced_array.push(Direction::North);
        }
    }
    else if coordinate.y < 0 {
        for i in (0..coordinate.y * -1) {
            reduced_array.push(Direction::South);
        }       
    }
    else {
    }

    return reduced_array;
}