use rand::RngExt;
use std::collections::HashSet;


fn generate_grid(height: usize, width: usize) -> Vec<Vec<i32>> {
    let mut grid = vec![vec![0i32; width]; height];
    for i in 0..height {
        for j in 0..width {
            grid[i][j] = 0;
        }
    }






    println!("The grid zeroed out");
    for i in 0..height {
        for j in 0..width {
            print!("{} ", grid[i][j]);
        }
        println!();
    }

    return grid;
}


//0 left
//1 right
//2 up
//3 down
fn step(row: usize, col: usize, height: usize, width: usize) -> (usize, usize) {
    loop {
        match rand::random_range(0..4) {
            0 if col > 0          => return (row, col - 1),
            1 if col + 1 < width  => return (row, col + 1),
            2 if row > 0          => return (row - 1, col),
            3 if row + 1 < height => return (row + 1, col),
            _ => {}
        }
    }
}

//This needs to return true if I have hit a position that is within the maze. So I can not continue the
fn traverse(grid: &Vec<Vec<i32>>, walk_pos_x: usize, walk_pos_y: usize, maze_tiles: HashSet<(usize, usize)>) -> Bool {
    let mut direction = walk();



}

fn generate_starts(height: usize, width: usize) -> (usize, usize, usize, usize) {
    let maze_row = rand::random_range(0..height);
    let maze_col = rand::random_range(0..width);
    loop {
        let walk_row = rand::random_range(0..height);
        let walk_col = rand::random_range(0..width);
        if (walk_row, walk_col) != (maze_row, maze_col) {
            return (maze_row, maze_col, walk_row, walk_col);
        }
    }
}


fn populate_maze(grid: &Vec<Vec<i32>>, width: usize, height: usize) -> () {

    let(maze_row_pos, maze_col_pos, walk_row_pos, walk_col_pos) = generate_starts(height, width);

    //now we walk from the walk start Until we hit the maze start
    let mut maze_tiles: HashSet<(usize, usize)> = HashSet::new();
    maze_tiles.insert(maze_row_pos, maze_col_pos);
    loop{
        if (maze_tiles.size() == height * width){
            break;
        }
        //now call traverse
        traverse(grid, walk_row_pos, walk_col_pos, maze_tiles);
    }


}


fn main(){
    println!("Please enter maze height:");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let height: usize = input.trim().parse().unwrap();

    println!("Please enter maze width:");

    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();

    let width: usize = input.trim().parse().unwrap();

    println!("Height: {}, Width: {}", height, width);


    let mut grid = generate_grid(height, width);
    populate_maze(&grid, height, width);



}