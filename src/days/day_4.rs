use bingo::Game;

pub fn main() {
    println!("### day 4 ###");

    let game = Game::from_file("./day_4.txt");
}

mod bingo {
    use std::fs;

    /// A bingo board.
    #[derive(Debug, Default)]
    struct Board([[(u8, bool); 5]; 5]);

    /// A bingo game.
    #[derive(Debug)]
    pub struct Game {
        numbers: Vec<u8>,
        boards: Vec<Board>,
    }

    impl Game {
        /// Constructs a bingo game from the file at `path`.
        ///
        /// # Panics
        ///
        /// Panics if there is a problem opening or reading the file.
        pub fn from_file(path: &str) -> Game {
            let contents = fs::read_to_string(path).unwrap();
            let lines = contents
                .lines()
                .map(|s| s.trim())
                .filter(|&s| !s.is_empty())
                .collect::<Vec<_>>();

            let numbers = lines
                .get(0)
                .unwrap()
                .split(',')
                .map(|s| s.parse::<u8>().unwrap())
                .collect::<Vec<_>>();

            let mut boards = Vec::with_capacity(lines.len());

            for raw_board in lines.get(1..).unwrap().chunks_exact(5) {
                let mut board = Board::default();

                for (i, &raw_row) in raw_board.iter().enumerate() {
                    for (j, num) in raw_row
                        .split_whitespace()
                        .map(|s| s.parse::<u8>().unwrap())
                        .enumerate()
                    {
                        board.0[i][j] = (num, false);
                    }
                }

                boards.push(board);
            }

            Game { numbers, boards }
        }
    }
}
