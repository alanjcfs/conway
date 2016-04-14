#[derive(Copy,Clone,Debug)]
enum Cell {
    O,
    X
}

fn main() {
    let mut table = [[Cell::O; 10]; 10];
    for row in table.into_iter() {
        println!("{:?}", row);
    }
}
