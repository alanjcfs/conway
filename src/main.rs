use std::fmt;
#[derive(Copy,Clone,Debug)]
enum State {
    Dead,
    Live
}

#[derive(Copy,Clone,Debug)]
struct Cell {
    x: u32,
    y: u32,
    state: State
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &State::Dead => write!(f, "{}", "0"),
            &State::Live => write!(f, "{}", "1")
        }
    }
}

fn main() {
    let mut table: Vec<Vec<Cell>> = vec![];
    for x in 0..10 {
        let mut row: Vec<Cell> = vec![];
        for y in 0..10 {
            row.push(Cell{x: x, y: y, state: State::Dead});
        }
        table.push(row);
    }

    /* Go through each cell and check its adjacent cells to determine if it is
     * alive or dead
     *   cccccccccc
     * i 0000000000
     * i 0000000000
     * i 0000000000
     * i 0000000000
     * i 0000000000
     * i 0000000000
     * i 0000000000
     * i 0000000000
     * i 0000000000
     * i 0000000000
     */

    for row in table.clone().into_iter() {
        for cell in row.into_iter() {
            print!("{}   ", cell.state);
        }
        println!("");
    }
    for row in table.clone().into_iter() {
        for cell in row.into_iter() {
            print!("{},{} ", cell.x, cell.y);
        }
        println!("");
    }
}
