use bingo::Game;

pub fn main() {
    println!("### day 4 ###");

    Game::from_file("./day_4.txt").play();
}

mod bingo {
    use std::collections::HashSet;
    use std::fs;

    /// The number of entries per row and column on the bingo board.
    const BOARD_LENGTH: usize = 5;

    /// A bingo square.
    #[derive(Debug, Default)]
    struct Square {
        number: u32,
        marked: bool,
    }

    /// A bingo board.
    #[derive(Debug, Default)]
    struct Board([[Square; BOARD_LENGTH]; BOARD_LENGTH]);

    impl Board {
        /// If `number` is present, it is marked and its coordinates are returned.
        fn mark(&mut self, number: u32) -> Option<(usize, usize)> {
            for i in 0..BOARD_LENGTH {
                for j in 0..BOARD_LENGTH {
                    let Square { number: n, .. } = self.0[i][j];
                    if n == number {
                        self.0[i][j].marked = true;
                        return Some((i, j));
                    }
                }
            }
            None
        }

        /// If this board is a winner, return its score wrapped in `Some`, else `None`.
        fn winner(&self) -> Option<u32> {
            for i in 0..BOARD_LENGTH {
                let mut is_winner = false;
                for j in 0..BOARD_LENGTH {
                    let Square { marked, .. } = self.0[i][j];
                    if marked {
                        is_winner = true;
                    } else {
                        is_winner = false;
                        break;
                    }
                }
                if is_winner {
                    return Some(self.score());
                }
            }

            for i in 0..BOARD_LENGTH {
                let mut is_winner = false;
                for j in 0..BOARD_LENGTH {
                    let Square { marked, .. } = self.0[j][i];
                    if marked {
                        is_winner = true;
                    } else {
                        is_winner = false;
                        break;
                    }
                }
                if is_winner {
                    return Some(self.score());
                }
            }

            None
        }

        /// Returns the score for the board by summing all unmarked squares.
        fn score(&self) -> u32 {
            let mut score = 0;
            for i in 0..BOARD_LENGTH {
                for j in 0..BOARD_LENGTH {
                    if !self.0[i][j].marked {
                        score += self.0[i][j].number;
                    }
                }
            }
            score
        }
    }

    /// A bingo game.
    #[derive(Debug)]
    pub struct Game {
        numbers: Vec<u32>,
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
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<_>>();

            let mut boards = Vec::with_capacity(lines.len());

            for raw_board in lines.get(1..).unwrap().chunks_exact(BOARD_LENGTH) {
                let mut board = Board::default();

                for (i, &raw_row) in raw_board.iter().enumerate() {
                    for (j, number) in raw_row
                        .split_whitespace()
                        .map(|s| s.parse::<u32>().unwrap())
                        .enumerate()
                    {
                        board.0[i][j] = Square {
                            number,
                            marked: false,
                        };
                    }
                }

                boards.push(board);
            }

            Game { numbers, boards }
        }

        /// Plays the bingo game.
        pub fn play(mut self) {
            let mut winners = HashSet::with_capacity(self.boards.len());

            for num in self.numbers {
                for (i, board) in self.boards.iter_mut().enumerate() {
                    if winners.get(&i).is_none() {
                        board.mark(num);

                        if let Some(winner_score) = board.winner() {
                            println!(
                                "board: {}, winner score: {}, final number: {}, product: {}",
                                i,
                                winner_score,
                                num,
                                winner_score * num
                            );
                            winners.insert(i);
                        }
                    }
                }
            }
        }
    }
}
