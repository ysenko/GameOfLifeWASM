use wasm_bindgen::prelude::*;
use js_sys::Math::random;
use fixedbitset::FixedBitSet;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

#[wasm_bindgen]
impl Universe {
    pub fn new(height: u32, width: u32) -> Universe {
        let cells = FixedBitSet::with_capacity((height * width) as usize);
        Universe {
            width,
            height,
            cells
        }
    }

    pub fn generate_predefined_universe_even_and_7(height: u32, width: u32) -> Universe {
        let mut universe = Universe::new(height, width);
        let mut live_cells= vec![];
        for row in 0..height {
            for col in 0..width {
                let idx = universe.get_index(row, col);
                if idx % 2 == 0 || idx % 7 == 0 {
                    live_cells.push((row, col));
                }
            }
        }
        universe.set_alive(live_cells.as_slice());
        universe
    }

    pub fn generate_random_universe(height: u32, width: u32) -> Universe {
        let mut universe = Universe::new(height, width);
        let mut live_cells= vec![];
        for row in 0..height {
            for col in 0..width {
                if random() > 0.5 {
                    live_cells.push((row, col));
                }
            }
        }
        universe.set_alive(live_cells.as_slice());
        universe
    }

    pub fn generate_simple_stable_universe(height: u32, width: u32) -> Universe {
        let mut universe = Universe::new(height, width);
        let live_cells= vec![(27, 28), (28, 28), (29, 28)];
        universe.set_alive(live_cells.as_slice());
        universe
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let is_alive = self.cells.contains(idx);
                let live_neighbor_count = self.live_neighbor_count(row, col);

                let next_cell = match (is_alive, live_neighbor_count) {
                    (true, x) if x < 2 => false,
                    (true, 2) | (true, 3) => true,
                    (true, x) if x > 3 => false,
                    (false, 3) => true,
                    (otherwise, _) => otherwise,
                };

                next.set(idx, next_cell);
            }
        }
        self.cells = next;
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }
}

impl Universe {
    pub fn set_alive(&mut self, alive_cells: &[(u32, u32)]) {
        for (row, col) in alive_cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells.set(idx, true);
        }
    }

    pub fn get_alive_cell_indexes(&self) -> Vec<(u32, u32)> {
        let mut live_cells: Vec<(u32, u32)> = vec![];
        for idx in self.cells.ones() {
            let row= idx as u32 / self.width;
            let col = idx as u32 - (self.width * row);
            live_cells.push((row, col));
        }
        live_cells
    }

    fn get_index(&self, row: u32, col: u32) -> usize {
        (self.width * row + col) as usize
    }

    fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
        let mut neighbor_count = 0;

        for row_delta in [self.height - 1, 0, 1].iter().clone() {
            for col_delta in [self.width - 1, 0, 1].iter().clone() {
                if *row_delta == 0 && *col_delta == 0 {
                    continue
                }
                let neighbor_row = (row + row_delta) % self.height;
                let neighbor_col = (col + col_delta) % self.width;

                neighbor_count += match self.cells.contains(self.get_index(neighbor_row, neighbor_col)) {
                    true => 1,
                    _ => 0
                }
            }
        }
        neighbor_count
    }

}
