use std::fmt;

fn main() {
}

struct Game {
    numbers: [Option<u8>; 81]
}

impl Game {
    fn new(numbers: [Option<u8>; 81]) -> Game {
        Game {
            numbers: numbers
        }
    }

    fn solve(&mut self) -> bool {
        loop {
            let mut did_mutate = false;
            let mut can_exist: Vec<u8> = Vec::with_capacity(9);
            for r in 0..9 {
                for c in 0..9 {
                    can_exist.drain(..);
                    if self.get(r, c).is_some() {
                        continue
                    }
                    let s = ((r / 3) * 3) + (c / 3);

                    // TODO: cache result of row/column/square calculation
                    let row = self.row(r);
                    let column = self.column(c);
                    let square = self.square(s);

                    for n in 1..10 {
                        if exists(&row, n) {
                            continue;
                        }
                        if exists(&column, n) {
                            continue;
                        }
                        if exists(&square, n) {
                            continue;
                        }

                        can_exist.push(n);
                    }
                    println!("{:?}", can_exist);

                    if can_exist.len() == 1 {
                        self.update(r, c, can_exist[0]);
                        did_mutate = true;
                    }
                }
            }

            if !did_mutate {
                return self.numbers.iter().all(|&n| n.is_some());
            }
        }
    }

    fn get(&self, row_index: u8, column_index: u8) -> Option<u8> {
        let index = ((row_index * 9) + column_index) as usize;
        self.numbers[index]
    }

    fn update(&mut self, row_index: u8, column_index: u8, value: u8) {
        let index = ((row_index * 9) + column_index) as usize;
        self.numbers[index] = Some(value);
    }


    fn row(&self, index: u8) -> [Option<u8>; 9] {
        let start = (index * 9) as usize;
        let end = start + 9;

        let mut coll = [None; 9];
        coll.clone_from_slice(&self.numbers[start..end]);

        coll
    }

    fn column(&self, index: u8) -> [Option<u8>; 9] {
        let mut coll = [None; 9];

        for i in 0..9usize {
            let j = (i * 9) + (index as usize);
            coll[i] = self.numbers[j];
        }

        coll
    }

    fn square(&self, index: u8) -> [Option<u8>; 9] {
        let verticle_offset = index / 3;
        let horizontal_offset = index % 3;

        let offset = ((verticle_offset * 27) + (horizontal_offset * 3)) as usize;

        let mut coll = [None; 9];

        for level in 0..3usize {
            let from = offset + (level * 9);
            let to = from + 3;
            for (i, val) in self.numbers[from..to].iter().enumerate() {
                coll[i + (level * 3)] = val.clone();
            }
        }

        coll
    }
}

impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, num) in self.numbers.iter().enumerate() {
            let display = if let Some(digit) = *num {
                format!("{}", digit)
            } else {
                String::from("*")
            };

            let result = if !((i + 1) % 9 == 0) {
                write!(f, "{} ", display)
            } else {
                write!(f, "{}\n", display)
            };

            if result.is_err() {
                return result;
            }
        }
        Ok(())
    }
}

fn exists<T: std::cmp::PartialEq + Copy>(collection: &[Option<T>], test: T) -> bool {
    for member in collection {
        if member.eq(&Some(test)) {
            return true
        }
    }
    false
}


#[cfg(test)]
mod tests {
    use super::Game;

    #[test]
    fn row_indexing() {
        let mut numbers = [None; 81];
        numbers[26] = Some(9);
        let game = Game::new(numbers);

        let mut expectation = [None; 9];
        expectation[8] = Some(9);

        assert!(game.row(2) == expectation)
    }

    #[test]
    fn column_indexing() {
        let mut numbers = [None; 81];
        numbers[12] = Some(9);
        let game = Game::new(numbers);

        let mut expectation = [None; 9];
        expectation[1] = Some(9);

        assert!(game.column(3) == expectation)
    }

    #[test]
    fn square_indexing() {
        let mut numbers = [None; 81];
        numbers[33] = Some(9);
        numbers[53] = Some(5);
        let game = Game::new(numbers);

        let mut expectation = [None; 9];
        expectation[0] = Some(9);
        expectation[8] = Some(5);

        assert!(game.square(5) == expectation)
    }

    #[test]
    fn solving_easy() {
        // Puzzle: http://www.puzzles.ca/sudoku_puzzles/sudoku_easy_245.html
        // Solution: http://www.puzzles.ca/sudoku_puzzles/sudoku_easy_245_solution.html

        let mut numbers = [None; 81];
        numbers[2]  = Some(2);
        numbers[11] = Some(5);
        numbers[12] = Some(8);
        numbers[14] = Some(2);
        numbers[15] = Some(9);
        numbers[19] = Some(8);
        numbers[21] = Some(4);
        numbers[22] = Some(6);
        numbers[25] = Some(3);
        numbers[27] = Some(9);
        numbers[31] = Some(7);
        numbers[33] = Some(2);
        numbers[35] = Some(6);
        numbers[39] = Some(9);
        numbers[41] = Some(1);
        numbers[55] = Some(6);
        numbers[57] = Some(1);
        numbers[63] = Some(1);
        numbers[64] = Some(7);
        numbers[67] = Some(5);
        numbers[70] = Some(4);
        numbers[71] = Some(8);
        numbers[72] = Some(2);
        numbers[75] = Some(3);
        numbers[76] = Some(4);
        numbers[77] = Some(6);
        let mut game = Game::new(numbers);


        let expectation = [
            Some(6), Some(1), Some(2), Some(7), Some(9), Some(3), Some(8), Some(5), Some(4),
            Some(3), Some(4), Some(5), Some(8), Some(1), Some(2), Some(9), Some(6), Some(7),
            Some(7), Some(8), Some(9), Some(4), Some(6), Some(5), Some(1), Some(3), Some(2),
            Some(9), Some(3), Some(1), Some(5), Some(7), Some(4), Some(2), Some(8), Some(6),
            Some(8), Some(5), Some(6), Some(9), Some(2), Some(1), Some(4), Some(7), Some(3),
            Some(4), Some(2), Some(7), Some(6), Some(3), Some(8), Some(5), Some(9), Some(1),
            Some(5), Some(6), Some(4), Some(1), Some(8), Some(7), Some(3), Some(2), Some(9),
            Some(1), Some(7), Some(3), Some(2), Some(5), Some(9), Some(6), Some(4), Some(8),
            Some(2), Some(9), Some(8), Some(3), Some(4), Some(6), Some(7), Some(1), Some(5)
        ];
        assert!(game.solve() == true);

        for (i,n) in game.numbers.iter().enumerate() {
            assert!(n.eq(&expectation[i]))
        }
    }

    #[test]
    fn solving_hard() {
        // Puzzle: http://www.puzzles.ca/sudoku_puzzles/sudoku_hard_243.html
        // Solution: http://www.puzzles.ca/sudoku_puzzles/sudoku_hard_243_solution.html
        let mut numbers = [None; 81];
        numbers[1]  = Some(2);
        numbers[2]  = Some(5);
        numbers[7]  = Some(6);
        numbers[9]  = Some(1);
        numbers[15] = Some(7);
        numbers[16] = Some(9);
        numbers[18] = Some(4);
        numbers[23] = Some(1);
        numbers[29] = Some(9);
        numbers[33] = Some(6);
        numbers[34] = Some(8);
        numbers[41] = Some(9);
        numbers[43] = Some(5);
        numbers[46] = Some(1);
        numbers[48] = Some(7);
        numbers[50] = Some(3);
        numbers[54] = Some(8);
        numbers[59] = Some(5);
        numbers[66] = Some(8);
        numbers[67] = Some(7);
        numbers[69] = Some(5);
        numbers[71] = Some(2);
        numbers[75] = Some(7);
        numbers[76] = Some(2);
        let mut game = Game::new(numbers);
        println!("");
        println!("{:?}", game);
        assert!(game.solve());
    }
}

