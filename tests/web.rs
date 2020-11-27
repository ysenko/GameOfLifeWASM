//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
extern crate wasm_game_of_life;
use wasm_bindgen_test::*;
use wasm_game_of_life::Universe;

wasm_bindgen_test_configure!(run_in_browser);

#[cfg(test)]
pub fn initial_universe() -> Universe {
    let mut universe = Universe::new(6, 6);
    universe.set_alive(&[(1, 3),(2, 3), (3, 3)]);
    universe
}

#[cfg(test)]
pub fn expected_universe() -> Universe {
    let mut universe = Universe::new(6, 6);
    universe.set_alive(&[(2, 2), (2, 3), (2, 4)]);
    universe
}

#[wasm_bindgen_test]
pub fn test_tick() {
    let mut initial_universe = initial_universe();
    let expected_universe = expected_universe();

    initial_universe.tick();

    assert_eq!(
        initial_universe.get_alive_cell_indexes(),
        expected_universe.get_alive_cell_indexes()
    );
}

#[wasm_bindgen_test]
pub fn test_width() {
    let universe = initial_universe();
    assert_eq!(universe.width(), 6);
}

#[wasm_bindgen_test]
pub fn test_height() {
    let universe = initial_universe();
    assert_eq!(universe.height(), 6);
}
