use wasm_bindgen::prelude::*;
use std::fmt;
use js_sys::Math::random;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Cell {
    Alive = 1,
    Dead = 0
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new(height: u32, width: u32) -> Universe {
        let cells = (0..width * height)
            .map(|_i| {
                Cell::Dead
            })
            .collect();
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

    pub fn generate_predefined_universe_random(height: u32, width: u32) -> Universe {
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

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbor_count = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbor_count) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
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

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
}

impl Universe {
    pub fn set_alive(&mut self, alive_cells: &[(u32, u32)]) {
        for (row, col) in alive_cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }

    pub fn get_alive_cell_indexes(&self) -> Vec<(u32, u32)> {
        let mut live_cells: Vec<(u32, u32)> = vec![];
        for cell in self.cells.iter().cloned().enumerate() {
            match cell {
                (idx, Cell::Alive) => {
                    let row= idx as u32 / self.width;
                    let col = idx as u32 - (self.width * row);
                    live_cells.push((row, col));
                },
                _ => {}
            };
        };
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

                neighbor_count += self.cells[self.get_index(neighbor_row, neighbor_col)] as u8;
            }
        }
        neighbor_count
    }

}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead {'◻'} else {'◼'};
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

