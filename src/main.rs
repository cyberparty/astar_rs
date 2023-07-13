mod data;
//mod pathfind;

use crate::data::board::Board;
fn main() {
    let mut board: Board = Board::new();
    board.load_from_file("src/plots_1.txt");
    println!("{}", board);
    println!("{:?}", board.get_neighbours((2, 1)));
    board.load_from_file("src/plots_2.txt");
    println!("{}", board);
    println!("{:?}", board.get_neighbours((2, 1)));
}
