/*
 | Name:   Joe Grimes, Michael Massenheimer
 | Course: CSC372
 | Assignment: Project Part 2
 | Instructor: L. McCann
 | Due Date: 11/16/2026 2:00 PM
 |
 | Description: This file serves to offer implementation for wilsons algorithm. Upon execution
 |              the user will be prompted to input height and width dimensions, and a grid will be
 |              generated from those dimensions, and a maze will be randomly populated.
 |
 | Language: rustc 1.1
 | Deficiencies: Know of no unsatisfied requirements and no logic errors
 */

use std::collections::HashSet;


/*
Name: generate_grid
Purpose: This method serves to generate a grid from a provided width and height. It will multiply
         the provided dimensions by 2 to accommodate for walls between maze spaces, and add one to each for the perimeter.
Pre-condition: Provided inputs should be valid usize inputs
Post-condition: A grid of dimensions 2width, 2height will be returned
Parameters: height, the provided height from the user
            width, the provided width from the user
Returns: The generated grid
*/

fn generate_grid(height: usize, width: usize) -> Vec<Vec<char>> {
    let grid = vec![vec!['#'; 2 * width + 1]; 2 * height + 1];
    return grid;
}



/*
Name: step
Purpose: This function serves to generate a random number 0-3, and attempt to step that direction in the grid
         Possible directions include left(0), right(1), up(2) down(3). If a step in that direction is within the bounds
         of provided width and height, and greater than 0, a tuple of the new position(x,y) is returned.
         If the new step is outside provided bounds, nothing is returned.
Pre-condition: Provided arguments should be valid usize and non negative
Post-condition: A tuple of the new position (or nothing) is returned.
Parameters: row, the current y position in the grid
            col, the current x position in the grid
            height, the height of the grid
            width, the width of the grid
Returns: A tuple of the new position, or nothing if step was illegal.
*/
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

/*
Name: traverse
Purpose: This function serves to take one random step from the current position. It will return true
         if the step lands on an existing maze tile (walk is over), false otherwise.
         If the step revisits a tile in the current walk, the loop in the current walk is erased.
Pre-condition: walk positions should be valid non negative indices, height and width should be non negative values
Post-condition: The current walk will end and the function will return true, or the new position will be added to the walk and return false.
Parameters: walk_pos_y, the current y position during the traverse
            walk_pos_x, the current x position during the traverse
            maze_tiles, set holding all tiles that are already included in the maze
            current_walk, the tiles of the current walk
            height, the height of the grid
            width, the width of the grid
Returns: bool, true if current walk has collided with tiles from the maze, or false if the walk will continue
*/
fn traverse(walk_pos_y: usize, walk_pos_x: usize, maze_tiles: &mut HashSet<(usize, usize)>, current_walk: &mut Vec<(usize, usize)>, height: usize, width: usize) -> bool {
    let new_pos = step(walk_pos_y, walk_pos_x, height, width);
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

/*
Name: generate starts
Purpose: This function serves to randomly generate starting positions for the maze, and new walk path
Pre-condition: height and width should be valid non negative usize
Post-condition: random indices for the starts of the walk path and maze will be generated
Parameters: height, the height of the grid
            width, the width of the grid
Returns: Tuple containing all 4 randomly generated indices for the walk and maze starting positions.
*/
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

/*
Name: persist changes to grid
Purpose: This method serves to copy the contents of the current walk into the maze tiles, and persist those positions into the existing grid.
         Walls between positions of the current walk are also torn down to show the connecting path.
Pre-condition: current walk should hold valid indices into the grid. The grid should be initialized with valid dimensions.
Post-condition: The current walk's contents will be added to maze tiles, and the grid will reflect the positions of the current walk
Parameters: current_walk, the most recent walk that is ready to be merged into the grid
            maze_tile, the set of tiles owned by the maze
            grid, the generated grid to hold the maze
Returns: None
*/
fn persist_changes_to_grid(current_walk: &mut Vec<(usize, usize)>, maze_tile: &mut HashSet<(usize, usize)>, grid: &mut Vec<Vec<char>>) -> (){
    for i in 0..current_walk.len() {
        maze_tile.insert(current_walk[i]);
        grid[2 * current_walk[i].0 + 1][2 * current_walk[i].1 + 1] = ' ';
    }

    for i in 1..current_walk.len() {
        let (row1, column1) = current_walk[i - 1];
        let (row2, column2) = current_walk[i];
        grid[row1 + row2 + 1][column1 + column2 + 1] = ' ';
    }
    //clear the current walk
    current_walk.clear();
}


/*
Name: populate maze
Purpose: This method serves to start the generation process of the maze. Random indices for the first maze and walk positions will be generated,
         then the function will repeatedly call traverse until every maze position has been occupied. The resulting grid will be printed to the screen.
Pre-condition: height and width should be valid non negative, grid should be initialized.
Post-condition: The maze will be generated and printed
Parameters: grid, the generated grid
            height, the height of the grid
            width, the width of the grid
Returns: None
*/
fn populate_maze(grid: &mut Vec<Vec<char>>, height: usize, width: usize) -> () {

    let (maze_row_pos, maze_col_pos, mut walk_row_pos, mut walk_col_pos) = generate_starts(height, width);

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