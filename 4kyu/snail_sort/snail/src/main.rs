fn main() {
    println!("Hello, world!");
    let square = &[
        vec![1, 2, 3, 4, 5, 7],
        vec![6, 7, 8, 9, 10, 8],
        vec![11, 12, 13, 14, 15,14],
        vec![16, 17, 18, 19, 20, 33],
        vec![21, 22, 23, 24, 25, 21],
        vec![21, 22, 23, 24, 25, 10],
    ];
    snail(square);
}

fn snail(matrix: &[Vec<i32>]) -> Vec<i32> {
    if matrix.len() == 1 {
        if matrix[0].len() == 0 {
            return vec![];
        }
        else {
            return vec![matrix[0][0]];
        }
    }
    if matrix.len() == 0 {
        return vec![];
    }
    let matrix_size = matrix.len();
    let mut one_demention_vec  = vec![];
    let mut right :usize   = matrix_size - 1;
    let mut down  :usize   = matrix_size;
    let mut left  :usize   = matrix_size;
    let mut up    :usize   = matrix_size;
    let mut up_coordinate  = (0, 0);
    loop {
        let right_coordinate = &go_right(matrix, &mut one_demention_vec, &up_coordinate, right);
        if up == 0 || down == 0    {break}
        up    -= 1;
        down  -= 1;
        let down_coordinate = &go_down(matrix, &mut one_demention_vec, right_coordinate, down);
        if right == 0 || left == 0 {break}
        right -= 1;
        left  -= 1;
        let left_coordinate  = &go_left(matrix, &mut one_demention_vec, down_coordinate, left);
        if down == 0 || up == 0    {break}
        down  -= 1;
        up    -= 1;
        up_coordinate = go_up(matrix, &mut one_demention_vec, left_coordinate, up);
        if left == 0 || right == 0 {break}
        left  -= 1;
        right -= 1;
    }
    println!("{:?}", one_demention_vec);
    return one_demention_vec;
}

fn go_right(matrix: &[Vec<i32>], one_demention_vec: &mut Vec<i32>, coordinate: &(usize, usize), steps: usize) -> (usize, usize) {
    // coordinate.0 - is LINE
    // coordinate.1 - is number of Vec
    let mut start = coordinate.0;
    let mut end   = coordinate.0+steps;
    for i in start..= end {
        one_demention_vec.push(matrix[coordinate.1][i]);
    }
    return (end, coordinate.1+1 );
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
    return (coordinate.0 + 1, coordinate.1-steps);
} 