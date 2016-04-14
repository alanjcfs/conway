use std::fmt;
#[derive(Copy,Clone,Debug)]
enum Cell {
    Dead,
    Live
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Cell::Dead => write!(f, "{}", "0"),
            &Cell::Live => write!(f, "{}", "1")
        }
    }
}

fn main() {
    let mut table = [[Cell::Dead; 10]; 10];
    table[0][0] = Cell::Live;
    table[0][1] = Cell::Live;

    /* Go through each cell and check its adjacent cells to determine if it is
     * alive or dead
     */


    for row in table.into_iter() {
        for cell in row.into_iter() {
            print!("{}", cell);
        }
        println!("");
    }
}
