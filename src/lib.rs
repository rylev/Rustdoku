use std::fmt;

#[derive(Clone)]
pub struct Game {
    pub numbers: [Option<u8>; 81],
}

impl Game {
    pub fn new(numbers: [Option<u8>; 81]) -> Game {
        Game { numbers: numbers }
    }

    pub fn full() -> Game {
        let numbers: [Option<u8>; 81] = [None; 81];
        let mut game = Game::new(numbers);
        game.solve();
        game
    }

    pub fn empty(&mut self, cell_count: u8) {
        let mut rng = random::Rng::new();

        for _ in 0..cell_count {
            let mut tries = 0;
            loop {
                let index = rng.gen_range(0, 81);
                let mut clone1 = self.clone();
                let mut clone2 = self.clone();
                if let Some(original) = self.numbers[index] {
                    clone1.numbers[index] = None;
                    clone2.numbers[index] = None;
                    if !clone1.solve_with_condition((index as u8, original)) && clone2.solve() {
                        self.numbers[index] = None;
                        break;
                    } else {
                        tries += 1;
                        if tries > 100 {
                            break;
                        }
                    }
                }
            }
        }
    }

    pub fn parse(string: &str) -> Option<Game> {
        if string.len() != 81 {
            return None;
        }

        let mut numbers = [None; 81];
        for (i, c) in string.chars().enumerate() {
            if c != '*' {
                match c.to_digit(10) {
                    Some(n) if n > 0 && n <= 9 => numbers[i] = Some(n as u8),
                    Some(_) => return None,
                    None => return None,
                }
            }
        }

        Some(Game::new(numbers))
    }

    pub fn solve(&mut self) -> bool {
        self.check(0, None)
    }

    pub fn solve_with_condition(&mut self, forbidden: (u8, u8)) -> bool {
        self.check(0, Some(forbidden))
    }

    fn check(&mut self, index: u8, forbidden: Option<(u8, u8)>) -> bool {
        if index > 80 {
            return true;
        }

        let mut rng = random::Rng::new();

        let original = self.numbers[index as usize];
        let mut possible_values = self.possible_values(index);
        loop {
            if possible_values.iter().all(|&v| v == false) {
                break;
            }
            let i = rng.gen_range(0, 9);
            let is_possible = possible_values[i];
            let value = (i + 1) as u8;
            let is_not_forbidden = match forbidden {
                None => true,
                Some((forbidden_index, forbidden_value)) => {
                    !(index == forbidden_index && value == forbidden_value)
                }
            };

            if is_possible {
                if is_not_forbidden {
                    self.numbers[index as usize] = Some(value);
                    if self.check(index + 1, forbidden) {
                        return true;
                    }
                }
                possible_values[i] = false;
            }
        }

        self.numbers[index as usize] = original;
        false
    }

    // Returns an array of size 9 where if the value at index i is true then i
    // is a possible value
    fn possible_values(&self, index: u8) -> [bool; 9] {
        if let Some(val) = self.numbers[index as usize] {
            let mut vals = [false; 9];
            vals[(val - 1) as usize] = true;
            return vals;
        }

        let row_index = index / 9;
        let column_index = index % 9;
        let square_index = ((row_index / 3) * 3) + (column_index / 3);
        let mut vals = [true; 9];

        let row = self.row(row_index);
        let column = self.column(column_index);
        let square = self.square(square_index);

        for &row_value in row.iter() {
            if let Some(i) = row_value {
                vals[(i - 1) as usize] = false;
            }
        }

        for &coll_value in column.iter() {
            if let Some(i) = coll_value {
                vals[(i - 1) as usize] = false;
            }
        }

        for &square_value in square.iter() {
            if let Some(i) = square_value {
                vals[(i - 1) as usize] = false;
            }
        }

        vals
    }

    fn row(&self, row_index: u8) -> [Option<u8>; 9] {
        let start = (row_index * 9) as usize;
        let end = start + 9;

        let mut coll = [None; 9];
        coll.clone_from_slice(&self.numbers[start..end]);

        coll
    }

    fn column(&self, coll_index: u8) -> [Option<u8>; 9] {
        let mut coll = [None; 9];

        for i in 0..9usize {
            let j = (i * 9) + (coll_index as usize);
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

    pub fn squares(&self) -> [[Option<u8>; 9]; 9] {
        let mut squares = [[None; 9]; 9];
        for square_index in 0..9 {
            let square = self.square(square_index);
            squares[square_index as usize] = square;
        }

        squares
    }
}

impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, num) in self.numbers.iter().enumerate() {
            if let &Some(digit) = num {
                write!(f, "{}", digit)?;
            } else {
                write!(f, "*")?;
            }

            if !((i + 1) % 9 == 0) {
                write!(f, " ")?;
            } else {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod random {
    pub struct Rng {
        inner: rand::ThreadRng,
    }

    impl Rng {
        pub fn new() -> Rng {
            Rng {
                inner: rand::thread_rng(),
            }
        }

        pub fn gen_range(&mut self, start: usize, end: usize) -> usize {
            use rand::Rng;
            self.inner.gen_range(start, end)
        }
    }
}

#[cfg(target_arch = "wasm32")]
mod random {
    pub struct Rng {}

    impl Rng {
        pub fn new() -> Rng {
            Rng {}
        }

        pub fn gen_range(&mut self, start: usize, end: usize) -> usize {
            let end = js_sys::Math::floor(js_sys::Math::random() * (end as f64)) as usize;
            start + end
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Game;

    #[test]
    fn test_successful_parsing() {
        let test =
            "***********1*******8********7***************************************************9";

        let game = Game::parse(test).unwrap();

        assert!(game.numbers[11] == Some(1));
        assert!(game.numbers[19] == Some(8));
        assert!(game.numbers[28] == Some(7));
        assert!(game.numbers[80] == Some(9));
    }

    #[test]
    fn test_parsing_with_wrong_number_of_chars() {
        let test1 =
            "***********1*******8********7***************************************************";
        assert!(test1.len() == 80);

        assert!(Game::parse(test1).is_none());

        let test2 =
            "***********1*******8********7*****************************************************";
        assert!(test2.len() == 82);

        assert!(Game::parse(test2).is_none());
    }

    #[test]
    fn test_parsing_with_wrong_chars() {
        let with_zero =
            "***********0*******8********7****************************************************";
        assert!(with_zero.len() == 81);

        assert!(Game::parse(with_zero).is_none());

        let with_letter =
            "***********a*******8********7****************************************************";
        assert!(with_letter.len() == 81);

        assert!(Game::parse(with_letter).is_none());

        let with_space =
            "*********** *******8********7****************************************************";
        assert!(with_space.len() == 81);

        assert!(Game::parse(with_space).is_none());
    }

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
        numbers[2] = Some(2);
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
            Some(6),
            Some(1),
            Some(2),
            Some(7),
            Some(9),
            Some(3),
            Some(8),
            Some(5),
            Some(4),
            Some(3),
            Some(4),
            Some(5),
            Some(8),
            Some(1),
            Some(2),
            Some(9),
            Some(6),
            Some(7),
            Some(7),
            Some(8),
            Some(9),
            Some(4),
            Some(6),
            Some(5),
            Some(1),
            Some(3),
            Some(2),
            Some(9),
            Some(3),
            Some(1),
            Some(5),
            Some(7),
            Some(4),
            Some(2),
            Some(8),
            Some(6),
            Some(8),
            Some(5),
            Some(6),
            Some(9),
            Some(2),
            Some(1),
            Some(4),
            Some(7),
            Some(3),
            Some(4),
            Some(2),
            Some(7),
            Some(6),
            Some(3),
            Some(8),
            Some(5),
            Some(9),
            Some(1),
            Some(5),
            Some(6),
            Some(4),
            Some(1),
            Some(8),
            Some(7),
            Some(3),
            Some(2),
            Some(9),
            Some(1),
            Some(7),
            Some(3),
            Some(2),
            Some(5),
            Some(9),
            Some(6),
            Some(4),
            Some(8),
            Some(2),
            Some(9),
            Some(8),
            Some(3),
            Some(4),
            Some(6),
            Some(7),
            Some(1),
            Some(5),
        ];

        println!("{:?}", game);
        println!("");
        assert!(game.solve());

        println!("{:?}", game);

        for (i, n) in game.numbers.iter().enumerate() {
            assert!(n.eq(&expectation[i]))
        }
    }

    #[test]
    fn solving_hard() {
        // Puzzle: http://www.puzzles.ca/sudoku_puzzles/sudoku_hard_243.html
        // Solution: http://www.puzzles.ca/sudoku_puzzles/sudoku_hard_243_solution.html
        let mut numbers = [None; 81];
        numbers[1] = Some(2);
        numbers[2] = Some(5);
        numbers[7] = Some(6);
        numbers[9] = Some(1);
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
        numbers[74] = Some(7);
        numbers[75] = Some(2);

        let mut game = Game::new(numbers);

        let expectation = [
            Some(9),
            Some(2),
            Some(5),
            Some(3),
            Some(4),
            Some(7),
            Some(1),
            Some(6),
            Some(8),
            Some(1),
            Some(6),
            Some(3),
            Some(5),
            Some(2),
            Some(8),
            Some(7),
            Some(9),
            Some(4),
            Some(4),
            Some(7),
            Some(8),
            Some(9),
            Some(6),
            Some(1),
            Some(2),
            Some(3),
            Some(5),
            Some(7),
            Some(3),
            Some(9),
            Some(4),
            Some(5),
            Some(2),
            Some(6),
            Some(8),
            Some(1),
            Some(2),
            Some(8),
            Some(4),
            Some(6),
            Some(1),
            Some(9),
            Some(3),
            Some(5),
            Some(7),
            Some(5),
            Some(1),
            Some(6),
            Some(7),
            Some(8),
            Some(3),
            Some(4),
            Some(2),
            Some(9),
            Some(8),
            Some(4),
            Some(2),
            Some(1),
            Some(3),
            Some(5),
            Some(9),
            Some(7),
            Some(6),
            Some(3),
            Some(9),
            Some(1),
            Some(8),
            Some(7),
            Some(6),
            Some(5),
            Some(4),
            Some(2),
            Some(6),
            Some(5),
            Some(7),
            Some(2),
            Some(9),
            Some(4),
            Some(8),
            Some(1),
            Some(3),
        ];

        assert!(game.solve());

        for (i, n) in game.numbers.iter().enumerate() {
            assert!(n.eq(&expectation[i]))
        }
    }

    #[test]
    fn generating_a_board() {
        let game = Game::full();

        for n in game.numbers.iter() {
            assert!(n.is_some())
        }
    }

    #[test]
    fn generating_a_board_with_5_removed() {
        let mut game = Game::full();
        game.empty(20);
        let mut count = 0;
        for n in game.numbers.iter() {
            if n.is_none() {
                count += 1;
            }
        }

        assert_eq!(count, 20);
        // for n in 1..10 {
        //     println!("--{}", n);
        //     for _ in 0..50 {
        //         let mut game = Game::full();
        //         let instant = std::time::Instant::now();
        //         game.empty(n);
        //         println!("{:?}", instant.elapsed());
        //     }
        // }
    }
}
