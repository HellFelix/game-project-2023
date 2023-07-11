use std::io::{self, Write};

use rand::{self, Rng};

#[derive(Debug, Clone, Copy)]
enum SquareState {
    None,
    Taken,

    Left,
    Right,
    Up,
    Down,
}

// each wall will be represented by a boolean.
// Either the path is blocked, or it isn't, pretty simple
#[derive(Clone)]
pub struct Maze {
    pub walls: (Vec<Vec<bool>>, Vec<Vec<bool>>),
}
impl Maze {
    pub fn gen(height: usize, width: usize) -> Self {
        wilsons_algorithm(height, width)
    }

    pub fn display_maze(&self) {
        let height = self.walls.0.len();
        let width = self.walls.1[0].len();

        // print top wall
        // there should be a gap in the top left corner
        print!(" ");
        for _ in 0..width - 1 {
            print!("___");
        }
        // the last one should leave a gap in the corner
        print!("__\n");
        io::stdout()
            .flush()
            .expect("Could not display Maze instance correctly");

        for h in 0..height {
            print!("|");
            for w in 0..width {
                // for the horizontal walls
                if h < height - 1 {
                    if self.walls.1[h][w] {
                        print!("__");
                    } else {
                        print!("  ");
                    }
                } else {
                    // lowest layer should be filled in
                    print!("__");
                }

                // for the vertical walls
                if w < width - 1 {
                    if self.walls.0[h][w] {
                        print!("|");
                    } else if (h < height - 1 && self.walls.1[h][w] && self.walls.1[h][w + 1])
                        || h == height - 1
                    {
                        // if both of the surrounding horizontal lines are filled in,
                        // it looks a bit strange if there's a gap between them
                        // this also applies if we're at the lowest level
                        print!("_");
                    } else {
                        print!(" ");
                    }
                }
            }
            print!("|\n");
            io::stdout()
                .flush()
                .expect("Could not display Maze instance correctly");
        }
    }
}

fn wilsons_algorithm(height: usize, width: usize) -> Maze {
    let mut grid: Vec<Vec<SquareState>> = vec![vec![SquareState::None; width]; height];
    let mut walls_horizontal: Vec<Vec<bool>> = vec![vec![true; width - 1]; height];
    let mut walls_vertical: Vec<Vec<bool>> = vec![vec![true; width]; height - 1];

    // starting square
    let starting_square = select_square(height, width, &grid);
    grid[starting_square.0][starting_square.1] = SquareState::Taken;

    // iterating over the grid to fill up the squares that are not already Taken
    while !is_complete(&grid) {
        fill_grid(
            &mut grid,
            (&mut walls_horizontal, &mut walls_vertical),
            height,
            width,
        );
    }

    Maze {
        walls: (walls_horizontal, walls_vertical),
    }
}

fn is_complete(grid: &Vec<Vec<SquareState>>) -> bool {
    for line in grid {
        for square in line {
            if let SquareState::None = square {
                return false;
            }
        }
    }
    // if the loop passes, the maze is complete
    true
}

// select a square to start the iteration from
fn select_square(height: usize, width: usize, grid: &Vec<Vec<SquareState>>) -> (usize, usize) {
    let square = (
        rand::thread_rng().gen_range(0..height),
        rand::thread_rng().gen_range(0..width),
    );

    // check to see if the square is taken
    if let SquareState::Taken = grid[square.0][square.1] {
        // if it is, then pick again
        select_square(height, width, grid)
    } else {
        square
    }
}

// select direction for random walk
fn select_direction(height: usize, width: usize, current_position: &(usize, usize)) -> SquareState {
    let rand_numb = rand::thread_rng().gen_range(1..=4);

    // check to make sure that we can move in the chosen direction
    if (current_position.1 == 0 && rand_numb == 1) // left
        || (current_position.1 == width - 1 && rand_numb == 2) // right
        || (current_position.0 == 0 && rand_numb == 3) // up
        || (current_position.0 == height - 1 && rand_numb == 4)
    // down
    {
        // pick again
        select_direction(height, width, current_position)
    } else {
        match rand_numb {
            1 => SquareState::Left,
            2 => SquareState::Right,
            3 => SquareState::Up,
            4 => SquareState::Down,
            // because the random number is generated on a range between 1 and 4, there is no way for
            // rand_numb to take on any other value
            _ => unreachable!(),
        }
    }
}

fn fill_grid(
    grid: &mut Vec<Vec<SquareState>>,
    walls: (&mut Vec<Vec<bool>>, &mut Vec<Vec<bool>>),
    height: usize,
    width: usize,
) {
    let starting_position = select_square(height, width, grid);
    let mut current_position = starting_position;

    // creating a path for this section of the maze
    loop {
        let direction = select_direction(height, width, &current_position);
        // set the direction of the current position
        grid[current_position.0][current_position.1] = direction;

        // set new current position
        match direction {
            SquareState::Right => {
                current_position.1 += 1;
            }
            SquareState::Left => {
                current_position.1 -= 1;
            }
            SquareState::Up => {
                current_position.0 -= 1;
            }
            SquareState::Down => {
                current_position.0 += 1;
            }
            // only the direction versions of the SquareState enum can be returned by the
            // select_direction function
            _ => unreachable!(),
        }
        if let SquareState::Taken = grid[current_position.0][current_position.1] {
            break;
        } else {
            continue;
        }
    }

    current_position = starting_position;

    // solidify the path and set out walls
    loop {
        match grid[current_position.0][current_position.1] {
            SquareState::Right => {
                walls.0[current_position.0][current_position.1] = false;
                grid[current_position.0][current_position.1] = SquareState::Taken;
                current_position.1 += 1;
            }
            SquareState::Left => {
                walls.0[current_position.0][current_position.1 - 1] = false;
                grid[current_position.0][current_position.1] = SquareState::Taken;
                current_position.1 -= 1;
            }
            SquareState::Down => {
                walls.1[current_position.0][current_position.1] = false;
                grid[current_position.0][current_position.1] = SquareState::Taken;
                current_position.0 += 1;
            }
            SquareState::Up => {
                walls.1[current_position.0 - 1][current_position.1] = false;
                grid[current_position.0][current_position.1] = SquareState::Taken;
                current_position.0 -= 1;
            }
            // we only need to cover the directions, there is no way for the current position to be
            // Taken or None
            _ => unreachable!(),
        }

        if let SquareState::Taken = grid[current_position.0][current_position.1] {
            break;
        } else {
            continue;
        }
    }

    for line in &mut *grid {
        for square in line {
            match square {
                SquareState::Taken => {}
                SquareState::None => {}
                _ => *square = SquareState::None,
            }
        }
    }
}

// strictly for testing purposes
#[allow(dead_code)]
fn display_grid(grid: &Vec<Vec<SquareState>>) {
    for line in grid {
        for square in line {
            print!("{square:?}\t\t");
        }
        print!("\n");
        io::stdout()
            .flush()
            .expect("Could not display the grid correctly");
    }
}
