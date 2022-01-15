mod board;
mod list;

use std::{io::{self, Read}, process};
use board::Board;
use smallvec::SmallVec;

use crate::list::parse_number_list;

fn main() {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();

    let input = String::from_utf8(input)
        .unwrap();

    let mut line = input
        .split("\n\n");

    let numbers = parse_number_list(line.next().unwrap());
    let mut boards = line.map(|line| Board::from_str(line)).collect::<Vec<Board>>();
    let mut won = Vec::new();

    for num in numbers {           
        let mut to_delete = Vec::new();
        for (i, board) in boards.iter_mut().enumerate() {
            board.extract(num);

            if board.winning() {
                won.push(board.score() * num);
                to_delete.push(i);
            }
        }

        for i in to_delete.iter().rev() {
            boards.remove(*i);
        }
    }

    println!("{:?}", won.last())
}







   // for num in numbers {
    //     boards
    //         .iter_mut()
    //         .for_each(|board| board.extract(num));
        
    //     boards
    //         .iter()
    //         .filter(|board| board.winning())
    //         .for_each(|board| {
    //             println!("{}\nwon!\n score: {}\n last_num: {}, solution: {}", board, board.score(), num, board.score() * num);
    //             won.push(board);
    //         });
        
    //         boards = boards
    //         .into_iter()
    //         .filter(|board| !board.winning())
    //         .collect();
        
    // }