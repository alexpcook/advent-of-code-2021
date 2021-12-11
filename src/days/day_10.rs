use std::fs;

pub fn main() {
    println!("### day 10 ###");

    let input = fs::read_to_string("./day_10.txt").expect("could not read input file");
    let chunks: Vec<&str> = input.lines().collect();

    let mut syntax_error_score = 0;
    for chunk in chunks.iter() {
        syntax_error_score += navigation::syntax_error_score(chunk);
    }

    println!("part 1: syntax error score = {}", syntax_error_score);

    let incomplete_chunks: Vec<_> = chunks
        .into_iter()
        .filter(|chunk| navigation::syntax_error_score(chunk) == 0)
        .collect();

    let mut scores = vec![];
    for chunk in incomplete_chunks {
        scores.push(navigation::autocomplete_score(chunk));
    }
    scores.sort_unstable();

    println!(
        "part 2: autocomplete score = {}",
        scores.get(scores.len() / 2).unwrap()
    );
}

mod navigation {
    /// Returns the syntax error score for `chunk`.
    /// 
    /// # Panics
    /// Panics if `chunk` contains any unknown characters.
    pub fn syntax_error_score(chunk: &str) -> u32 {
        let mut s = String::with_capacity(chunk.len());

        for c in chunk.chars() {
            if is_open_chunk_char(c) {
                s.push(c)
            } else if is_close_chunk_char(c) {
                if let Some(last) = s.chars().last() {
                    if last != get_open_chunk_char(c) {
                        return get_syntax_error_score_for_char(c);
                    } else {
                        s.pop();
                    }
                } else {
                    return get_syntax_error_score_for_char(c);
                }
            } else {
                panic!("unknown chunk character: {}", c)
            }
        }

        0
    }

    /// Returns the autocomplete score for `chunk`. Assumes `chunk` is valid.
    pub fn autocomplete_score(chunk: &str) -> u64 {
        let mut v = vec![];
        for c in chunk.chars() {
            if is_open_chunk_char(c) {
                v.push(c);
            } else {
                v.pop();
            }
        }

        let mut score = 0;
        for c in v.into_iter().rev() {
            score *= 5;
            score += get_autocomplete_score_for_char(get_close_chunk_char(c));
        }

        score
    }

    /// Returns whether `c` is an open chunk character.
    fn is_open_chunk_char(c: char) -> bool {
        matches!(c, '(' | '[' | '{' | '<')
    }

    /// Returns whether `c` is a close chunk character.
    fn is_close_chunk_char(c: char) -> bool {
        matches!(c, ')' | ']' | '}' | '>')
    }

    /// Given a close chunk character `c`, returns its open chunk character.
    ///
    /// # Panics
    /// Panics if `c` is not a known close chunk character.
    fn get_open_chunk_char(c: char) -> char {
        match c {
            ')' => '(',
            ']' => '[',
            '}' => '{',
            '>' => '<',
            _ => panic!("unknown close chunk character: {}", c),
        }
    }

    /// Given an open chunk character `c`, returns its close chunk character.
    ///
    /// # Panics
    /// Panics if `c` is not a known open chunk character.
    fn get_close_chunk_char(c: char) -> char {
        match c {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => panic!("unknown open chunk character: {}", c),
        }
    }

    /// Given a close chunk character `c`, returns its syntax error score.
    ///
    /// # Panics
    /// Panics if `c` is not a known close chunk character.
    fn get_syntax_error_score_for_char(c: char) -> u32 {
        match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!("unknown close chunk character: {}", c),
        }
    }

    /// Given a close chunk character `c`, returns its autocomplete score.
    ///
    /// # Panics
    /// Panics if `c` is not a known close chunk character.
    fn get_autocomplete_score_for_char(c: char) -> u64 {
        match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!("unknown close chunk character: {}", c),
        }
    }
}
