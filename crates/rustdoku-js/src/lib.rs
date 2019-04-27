use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone)]
pub struct Game {
    inner: sudoku::Game,
}

#[wasm_bindgen]
pub struct SolveResult {
    game: Game,
    steps: usize,
}
#[wasm_bindgen]
impl SolveResult {
    pub fn game(&self) -> Game {
        self.game.clone()
    }
    pub fn steps(&self) -> usize {
        self.steps
    }
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(count_missing: u8) -> Game {
        console_error_panic_hook::set_once();
        loop {
            let mut game = Game::new_full();
            if game.empty(count_missing) {
                return game;
            }
        }
    }

    #[wasm_bindgen(js_name = avgSteps)]
    pub fn avg_steps(&mut self, runs: usize) -> f64 {
        (0..runs).map(|_| self.solve().steps).sum::<usize>() as f64 / runs as f64
    }

    pub fn from(numbers: Vec<JsValue>) -> Game {
        console_error_panic_hook::set_once();
        let mut result = [None; 81];
        for (i, v) in numbers.iter().take(81).enumerate() {
            if !v.is_undefined() {
                result[i] = Some(v.as_f64().unwrap() as u8);
            }
        }
        Game {
            inner: sudoku::Game { numbers: result },
        }
    }

    #[wasm_bindgen(js_name = newFull)]
    pub fn new_full() -> Game {
        let mut game = sudoku::Game::new([None; 81]);
        let (solved, _) = game.solve();
        assert!(solved);
        Game { inner: game }
    }

    #[wasm_bindgen(js_name = newEasy)]
    pub fn new_easy() -> Game {
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
        let inner = sudoku::Game::new(numbers);
        Game { inner }
    }

    #[wasm_bindgen(js_name = newHard)]
    pub fn new_hard() -> Game {
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

        let inner = sudoku::Game::new(numbers);
        Game { inner }
    }

    pub fn empty(&mut self, count: u8) -> bool {
        self.inner.empty(count)
    }

    pub fn solve(&mut self) -> SolveResult {
        let clone = Game {
            inner: self.inner.clone(),
        };
        let (_, steps) = self.inner.solve();
        let solved = std::mem::replace(self, clone);

        SolveResult {
            game: solved,
            steps,
        }
    }

    #[wasm_bindgen]
    pub fn squares(&self) -> Vec<wasm_bindgen::JsValue> {
        self.inner
            .squares()
            .iter()
            .map(|s| {
                let array = js_sys::Array::new();
                let vec: Vec<_> = s.iter().map(Self::cell_to_js).collect();
                for item in vec {
                    array.push(&item);
                }
                array.into()
            })
            .collect()
    }

    #[wasm_bindgen(js_name = toJs)]
    pub fn to_js(&self) -> Vec<wasm_bindgen::JsValue> {
        self.inner.numbers.iter().map(Self::cell_to_js).collect()
    }

    fn cell_to_js(cell: &Option<u8>) -> wasm_bindgen::JsValue {
        match cell {
            &None => wasm_bindgen::JsValue::UNDEFINED,
            &Some(n) => wasm_bindgen::JsValue::from_f64(n as f64),
        }
    }
}
