extern crate fixedbitset;
extern crate js_sys;

use crate::utils;

use fixedbitset::FixedBitSet;
use js_sys::Math;
use std::fmt;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
pub struct Delta {
    state: bool,
    index: usize,
}

impl Delta {
    pub fn new(state: bool, index: usize) -> Delta {
        Delta { state, index }
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
    creation_rate: f64, // smaller type possible ?
    next_gen: Vec<Delta>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        utils::set_panic_hook();

        let width = 120;
        let height = 120;
        let creation_rate = 0.85;
        let size = (width * height) as usize;
        let next_gen = Vec::new();

        let mut cells = FixedBitSet::with_capacity(size);

        for i in 0..size {
            cells.set(i, Math::random() > creation_rate);
        }

        Universe {
            width,
            height,
            cells,
            creation_rate,
            next_gen,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    pub fn tick(&mut self) {
        // let _timer = Timer::new("Universe::tick");
        let mut deltas: Vec<Delta> = Vec::new();
        let mut next = {
            // let _timer = Timer::new("allocate next cells");
            self.cells.clone()
        };
        {
            // let _timer = Timer::new("new generation");
            for row in 0..self.height {
                for col in 0..self.width {
                    let idx = self.get_index(row, col);
                    let cell = self.cells[idx];
                    let live_neighbors = self.live_neighbor_count(row, col);
                    let next_cell = match (cell, live_neighbors) {
                        (true, x) if x < 2 => false,
                        (true, 2) | (true, 3) => true,
                        (true, x) if x > 3 => false,
                        (false, 3) => true,
                        (otherwise, _) => otherwise,
                    };

                    if next_cell != cell {
                        deltas.push(Delta {
                            state: next_cell,
                            index: idx,
                        });
                    }

                    next.set(idx, next_cell);
                }
            }
        }

        // let _timer = Timer::new("free old cells");
        self.next_gen = deltas;
        self.cells = next;
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        let size = (width * self.height) as usize;
        self.cells = FixedBitSet::with_capacity(size);
        for i in 0..size {
            self.cells.set(i, Math::random() > self.creation_rate);
        }
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        let size = (self.width * height) as usize;
        self.cells = FixedBitSet::with_capacity(size);
        for i in 0..size {
            self.cells.set(i, Math::random() > self.creation_rate);
        }
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    pub fn next_gen(&self) -> JsValue {
        JsValue::from_serde(&self.next_gen).unwrap()
    }

    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        match self.cells[idx] {
            true => self.cells.set(idx, false),
            false => self.cells.set(idx, true),
        }
    }

    pub fn randomize(&mut self) {
        let size = (self.width * self.height) as usize;
        for i in 0..size {
            self.cells.set(i, Math::random() > self.creation_rate);
        }
    }

    pub fn blank(&mut self) {
        let size = (self.height * self.width) as usize;
        for i in 0..size {
            self.cells.set(i, false);
        }

        self.next_gen = Vec::new();
    }
}

impl Universe {
    pub fn get_cells(&self) -> &FixedBitSet {
        &self.cells
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells.set(idx, true);
        }
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;

        // Actually faster than using modulo
        let north = if row == 0 { self.height - 1 } else { row - 1 };

        let south = if row == self.height - 1 { 0 } else { row + 1 };

        let west = if column == 0 {
            self.width - 1
        } else {
            column - 1
        };

        let east = if column == self.width - 1 {
            0
        } else {
            column + 1
        };

        let nw = self.get_index(north, west);
        count += self.cells[nw] as u8;

        let n = self.get_index(north, column);
        count += self.cells[n] as u8;

        let ne = self.get_index(north, east);
        count += self.cells[ne] as u8;

        let w = self.get_index(row, west);
        count += self.cells[w] as u8;

        let e = self.get_index(row, east);
        count += self.cells[e] as u8;

        let sw = self.get_index(south, west);
        count += self.cells[sw] as u8;

        let s = self.get_index(south, column);
        count += self.cells[s] as u8;

        let se = self.get_index(south, east);
        count += self.cells[se] as u8;

        count
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == 0 { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
