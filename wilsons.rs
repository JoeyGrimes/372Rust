use std::collections::HashSet;



/*---------------------------------------------------------------------
        |  Method
        |
        |  Purpose:

        |  Pre-condition:
        |
        |  Post-condition:
        |
        |  Parameters: row
        |
        |  Returns:  None.
        *-------------------------------------------------------------------*/
fn generate_grid(height: usize, width: usize) -> Vec<Vec<char>> {
    let grid = vec![vec!['#'; 2 * width + 1]; 2 * height + 1];
    return grid;
}


//0 left
//1 right
//2 up
//3 down
/*---------------------------------------------------------------------
        |  Method
        |
        |  Purpose:

        |  Pre-condition:
        |
        |  Post-condition:
        |
        |  Parameters: row
        |
        |  Returns:  None.
        *-------------------------------------------------------------------*/
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

/*---------------------------------------------------------------------
        |  Method
        |
        |  Purpose:

        |  Pre-condition:
        |
        |  Post-condition:
        |
        |  Parameters: row
        |
        |  Returns:  None.
        *-------------------------------------------------------------------*/
fn traverse(walk_pos_x: usize, walk_pos_y: usize, maze_tiles: &mut HashSet<(usize, usize)>, current_walk: &mut Vec<(usize, usize)>, height: usize, width: usize) -> bool {
    let new_pos = step(walk_pos_x, walk_pos_y, height, width);
    if current_walk.contains(&new_pos) {
        let repeated_index = current_walk.iter().position(|x| *x == new_pos).unwrap();
        current_walk.truncate(repeated_index + 1);
        return false;
    }

    current_walk.push(new_pos);

    if maze_tiles.contains(&new_pos) {
        return true;
    }
    return false;
}

/*---------------------------------------------------------------------
        |  Method
        |
        |  Purpose:

        |  Pre-condition:
        |
        |  Post-condition:
        |
        |  Parameters: row
        |
        |  Returns:  None.
        *-------------------------------------------------------------------*/
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

/*---------------------------------------------------------------------
        |  Method
        |
        |  Purpose:

        |  Pre-condition:
        |
        |  Post-condition:
        |
        |  Parameters: row
        |
        |  Returns:  None.
        *-------------------------------------------------------------------*/
fn persist_changes_to_grid(current_walk: &mut Vec<(usize, usize)>, maze_tile: &mut HashSet<(usize, usize)>, grid: &mut Vec<Vec<char>>) -> (){

    for i in 0..current_walk.len() {
        maze_tile.insert(current_walk[i]);
        grid[2 * current_walk[i].0 + 1][2 * current_walk[i].1 + 1] = ' ';
    }

    for i in 1..current_walk.len() {
        let (r1, c1) = current_walk[i - 1];
        let (r2, c2) = current_walk[i];
        grid[r1 + r2 + 1][c1 + c2 + 1] = ' ';
    }
    current_walk.clear();
}

/*---------------------------------------------------------------------
        |  Method
        |
        |  Purpose:

        |  Pre-condition:
        |
        |  Post-condition:
        |
        |  Parameters: row
        |
        |  Returns:  None.
        *-------------------------------------------------------------------*/
fn populate_maze(grid: &mut Vec<Vec<char>>, width: usize, height: usize) -> () {

    let (mut maze_row_pos, mut maze_col_pos, mut walk_row_pos, mut walk_col_pos) = generate_starts(height, width);

    let mut maze_tiles: HashSet<(usize, usize)> = HashSet::new();
    let mut current_walk: Vec<(usize, usize)> = Vec::new();
    maze_tiles.insert((maze_row_pos, maze_col_pos));
    grid[2 * maze_row_pos + 1][2 * maze_col_pos + 1] = ' ';
    current_walk.push((walk_row_pos, walk_col_pos));
    loop {
        let is_current_walk_over = traverse(walk_row_pos, walk_col_pos, &mut maze_tiles, &mut current_walk, height, width);
        if is_current_walk_over {
            persist_changes_to_grid(&mut current_walk, &mut maze_tiles, grid);

            if maze_tiles.len() == height * width {
                break;
            }

            loop {
                walk_row_pos = rand::random_range(0..height);
                walk_col_pos = rand::random_range(0..width);
                if !maze_tiles.contains(&(walk_row_pos, walk_col_pos)) {
                    break;
                }
            }
            current_walk.push((walk_row_pos, walk_col_pos));
        } else {
            (walk_row_pos, walk_col_pos) = *current_walk.last().unwrap();
        }
    }

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            print!("{} ", grid[i][j]);
        }
        println!();
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
    populate_maze(&mut grid, height, width);



}