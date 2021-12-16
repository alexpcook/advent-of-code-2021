use origami::Origami;
use std::path::Path;

pub fn main() {
    println!("### day 13 ###");

    let input = Path::new("./day_13.txt");
    let mut origami = Origami::from_file(input).expect("could not read input file");

    if origami.fold().is_some() {
        println!(
            "part 1: dots visible after first fold = {}",
            origami.visible()
        );
    }

    while origami.fold().is_some() {}
    println!("part 2: infrared thermal imaging camera system code");
    println!("{}", origami);
}

mod origami {
    use std::collections::{HashSet, VecDeque};
    use std::fmt;
    use std::fs;
    use std::io;
    use std::path::Path;

    /// The origami paper.
    #[derive(Debug)]
    struct Paper {
        dots: HashSet<(usize, usize)>,
        x_max: usize,
        y_max: usize,
    }

    /// The axis of a fold instruction.
    #[derive(Debug)]
    enum Axis {
        X,
        Y,
    }

    /// A fold instruction.
    #[derive(Debug)]
    pub struct Instruction {
        axis: Axis,
        line: usize,
    }

    /// The origami paper with fold instructions.
    #[derive(Debug)]
    pub struct Origami {
        paper: Paper,
        instructions: VecDeque<Instruction>,
    }

    impl Origami {
        /// Constructs an `Origami` instance from input `path`.
        pub fn from_file(path: &Path) -> io::Result<Origami> {
            let mut origami = Origami {
                paper: Paper {
                    dots: HashSet::new(),
                    x_max: 0,
                    y_max: 0,
                },
                instructions: VecDeque::new(),
            };

            let contents = fs::read_to_string(path)?;
            for line in contents.lines() {
                if line.starts_with("fold along") {
                    let mut pieces = line.splitn(2, '=');

                    let axis = pieces
                        .next()
                        .map(|s| match s.chars().last() {
                            Some('x') => Axis::X,
                            Some('y') => Axis::Y,
                            _ => panic!("invalid instruction axis"),
                        })
                        .unwrap();
                    let line = match pieces.next() {
                        Some(i) => i.parse::<usize>().unwrap(),
                        _ => panic!("invalid instruction line"),
                    };

                    origami.instructions.push_back(Instruction { axis, line })
                } else if !line.is_empty() {
                    let mut dot = line.splitn(2, ',').map(|s| s.parse::<usize>().unwrap());
                    let x = dot.next().unwrap();
                    let y = dot.next().unwrap();

                    origami.paper.dots.insert((x, y));

                    if x > origami.paper.x_max {
                        origami.paper.x_max = x;
                    }

                    if y > origami.paper.y_max {
                        origami.paper.y_max = y;
                    }
                }
            }

            Ok(origami)
        }

        /// Folds the `Origami` instance according to the next instruction.
        /// Returns `None` if no instructions remain, else returns `Some` containing
        /// the instruction that was executed.
        pub fn fold(&mut self) -> Option<Instruction> {
            let instruction = self.instructions.pop_front()?;

            match instruction.axis {
                Axis::X => {
                    self.paper.x_max = if self.paper.x_max > instruction.line {
                        self.paper.x_max - instruction.line - 1
                    } else {
                        instruction.line - 1
                    };

                    let mut new_dots = HashSet::with_capacity(self.paper.dots.len());

                    for (x, y) in self.paper.dots.drain() {
                        if x < instruction.line {
                            new_dots.insert((x, y));
                        } else {
                            new_dots.insert((x - 2 * (x - instruction.line), y));
                        }
                    }

                    self.paper.dots = new_dots;
                }
                Axis::Y => {
                    self.paper.y_max = if self.paper.y_max > instruction.line {
                        self.paper.y_max - instruction.line - 1
                    } else {
                        instruction.line - 1
                    };

                    let mut new_dots = HashSet::with_capacity(self.paper.dots.len());

                    for (x, y) in self.paper.dots.drain() {
                        if y < instruction.line {
                            new_dots.insert((x, y));
                        } else {
                            new_dots.insert((x, y - 2 * (y - instruction.line)));
                        }
                    }

                    self.paper.dots = new_dots;
                }
            }

            Some(instruction)
        }

        /// Returns the number of dots visible.
        pub fn visible(&self) -> usize {
            self.paper.dots.len()
        }
    }

    impl fmt::Display for Origami {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            for y in 0..=self.paper.y_max {
                for x in 0..=self.paper.x_max {
                    write!(
                        f,
                        "{}",
                        if self.paper.dots.contains(&(x, y)) {
                            "#"
                        } else {
                            "."
                        }
                    )?;
                }
                writeln!(f)?;
            }
            Ok(())
        }
    }
}
