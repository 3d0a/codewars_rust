fn main() {
    println!("Hello, world!");
    let square = &[
        vec![1,2,3],
        vec![4,5,6],
        vec![7,8,9],
    ];
    snail(square);
}

fn snail(matrix: &[Vec<i32>]) -> Vec<i32> {
    let matrix_size = matrix.len() - 1;
    let mut one_demention_vec  = vec![];
    let mut right :usize = matrix_size;
    let mut down  :usize = matrix_size;
    let mut left  :usize = matrix_size;
    let mut up    :usize = matrix_size;
    let up_coordinate = &(0, 0);
    loop {
        let right_coordinate = &go_right(matrix, &mut one_demention_vec, up_coordinate, right);
        up    -= 1;
        if up == 0 {break}
        let down_coordinate  = &go_down(matrix, &mut one_demention_vec, right_coordinate, down);
        right -= 1;
        if right == 0 {break}
        let left_coordinate  = &go_left(matrix, &mut one_demention_vec, down_coordinate, left);
        down  -= 1;
        if down == 0 {break}
        let up_coordinate    = &go_up(matrix, &mut one_demention_vec, left_coordinate, up);
        left  -= 1;
        if left == 0 {break}
    }
    println!("{:?}", one_demention_vec);
    return one_demention_vec;
}

fn go_right(matrix: &[Vec<i32>], one_demention_vec: &mut Vec<i32>, coordinate: &(usize, usize), steps: usize) -> (usize, usize) {
    // coordinate.0 - is LINE
    // coordinate.1 - is number of Vec
    for i in coordinate.0..=coordinate.0+steps {
        one_demention_vec.push(matrix[coordinate.1][i]);
    }
    return (coordinate.0+steps, coordinate.1+1 );
} 

fn go_down(matrix: &[Vec<i32>], one_demention_vec: &mut Vec<i32>, coordinate: &(usize, usize), steps: usize) -> (usize, usize) {
    // coordinate.0 - is LINE
    // coordinate.1 - is number of Vec
    for i in coordinate.1..=coordinate.1 + steps - 1 {
        one_demention_vec.push(matrix[i][coordinate.0]);
    }
    return (coordinate.0, coordinate.1+steps - 1);
} 

fn go_left(matrix: &[Vec<i32>], one_demention_vec: &mut Vec<i32>, coordinate: &(usize, usize), steps: usize) -> (usize, usize) {
    // coordinate.0 - is LINE
    // coordinate.1 - is number of Vec
    for i in (coordinate.0-steps..coordinate.0).rev() {
        one_demention_vec.push(matrix[coordinate.1][i]);
    }
    return (coordinate.0 - steps, coordinate.1);
} 

fn go_up(matrix: &[Vec<i32>], one_demention_vec: &mut Vec<i32>, coordinate: &(usize, usize), steps: usize) -> (usize, usize) {
    // coordinate.0 - is LINE
    // coordinate.1 - is number of Vec
    for i in (coordinate.1-steps..=coordinate.1 - 1).rev() {
        one_demention_vec.push(matrix[i][coordinate.0]);
    }
    return (coordinate.0, coordinate.1-steps);
} 