extern crate rand;

use std::mem;
use rand::Rng;

pub struct World {
    width: i16,
    height: i16,
    cells: Vec<bool>,
    scratch_cells: Vec<bool>,
}

impl World {
    pub fn new(width: i16, height: i16) -> World {
        World {
            width: width,
            height: height,
            cells: vec![false; (width as usize) * (height as usize)],
            scratch_cells: vec![false; (width as usize) * (height as usize)],
        }
    }

    fn next_cell_state(live: bool, neighbor_count: u8) -> bool {
        neighbor_count == 3 || (neighbor_count == 2 && live)
    }

    pub fn populate_rand(&mut self, live_probability: f64) {
        let mut rng = rand::thread_rng();

        for y in 0..self.height {
            for x in 0..self.width {
                let live = rng.gen::<f64>() <= live_probability;
                self.set(x, y, live)
            }
        }
    }

    pub fn width(&self) -> i16 { self.width }
    pub fn height(&self) -> i16 { self.height }

    pub fn get(&self, x: i16, y: i16) -> bool {
        self.cells[self.coord_to_idx(x, y)]
    }

    pub fn set(&mut self, x: i16, y: i16, val: bool) {
        let idx = self.coord_to_idx(x, y);
        self.cells[idx] = val;
    }

    pub fn step(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = self.coord_to_idx(x, y);
                self.scratch_cells[idx] = World::next_cell_state(self.get(x, y), self.count_neighbors(x, y));
            }
        }
        mem::swap(&mut self.cells, &mut self.scratch_cells);
    }

    fn coord_to_idx(&self, x: i16, y: i16) -> usize {
        let mut x = x % self.width;
        if x < 0 {
            x += self.width;
        }
        let mut y = y % self.height;
        if y < 0 {
            y += self.height;
        }

        (y as usize) * (self.width as usize) + (x as usize)
    }

    fn count_neighbors(&self, x: i16, y: i16) -> u8 {
        (self.get(x-1, y-1) as u8) +
        (self.get(x, y-1) as u8) +
        (self.get(x+1, y-1) as u8) +
        (self.get(x-1, y) as u8) +
        (self.get(x+1, y) as u8) +
        (self.get(x-1, y+1) as u8) +
        (self.get(x, y+1) as u8) +
        (self.get(x+1, y+1) as u8)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_and_set() {
        let mut world = World::new(8, 8);
        assert_eq!(false, world.get(0, 0));

        world.set(0, 0, true);
        assert_eq!(true, world.get(0, 0));
    }

    #[test]
    fn wrap_coord() {
        let mut world = World::new(8, 8);
        assert_eq!(false, world.get(-1, -1));
        assert_eq!(false, world.get(7, 7));

        world.set(-1, -1, true);
        assert_eq!(true, world.get(-1, -1));
        assert_eq!(true, world.get(7, 7));
    }

    #[test]
    fn count_neighbors() {
        let mut world = World::new(8, 8);
        assert_eq!(0, world.count_neighbors(1, 1));

        world.set(0, 0, true);
        assert_eq!(1, world.count_neighbors(1, 1));

        world.set(1, 0, true);
        assert_eq!(2, world.count_neighbors(1, 1));

        world.set(2, 0, true);
        assert_eq!(3, world.count_neighbors(1, 1));

        world.set(0, 1, true);
        assert_eq!(4, world.count_neighbors(1, 1));

        world.set(2, 1, true);
        assert_eq!(5, world.count_neighbors(1, 1));

        world.set(0, 2, true);
        assert_eq!(6, world.count_neighbors(1, 1));

        world.set(1, 2, true);
        assert_eq!(7, world.count_neighbors(1, 1));

        world.set(2, 2, true);
        assert_eq!(8, world.count_neighbors(1, 1));

        // Self doesn't count
        world.set(1, 1, true);
        assert_eq!(8, world.count_neighbors(1, 1));
    }

    #[test]
    fn next_cell_state() {
        assert_eq!(false, World::next_cell_state(false, 0));
        assert_eq!(false, World::next_cell_state(false, 1));
        assert_eq!(false, World::next_cell_state(false, 2));
        assert_eq!(true, World::next_cell_state(false, 3));
        assert_eq!(false, World::next_cell_state(false, 4));
        assert_eq!(false, World::next_cell_state(false, 5));
        assert_eq!(false, World::next_cell_state(false, 6));
        assert_eq!(false, World::next_cell_state(false, 7));
        assert_eq!(false, World::next_cell_state(false, 8));

        assert_eq!(false, World::next_cell_state(true, 0));
        assert_eq!(false, World::next_cell_state(true, 1));
        assert_eq!(true, World::next_cell_state(true, 2));
        assert_eq!(true, World::next_cell_state(true, 3));
        assert_eq!(false, World::next_cell_state(true, 4));
        assert_eq!(false, World::next_cell_state(true, 5));
        assert_eq!(false, World::next_cell_state(true, 6));
        assert_eq!(false, World::next_cell_state(true, 7));
        assert_eq!(false, World::next_cell_state(true, 8));
    }

    #[test]
    fn step() {
        let mut world = World::new(8, 8);

        world.set(0, 1, true);
        world.set(1, 1, true);
        world.set(2, 1, true);

        world.step();

        assert_eq!(false, world.get(0, 0));
        assert_eq!(true, world.get(1, 0));
        assert_eq!(false, world.get(2, 0));
        assert_eq!(false, world.get(0, 1));
        assert_eq!(true, world.get(1, 1));
        assert_eq!(false, world.get(2, 1));
        assert_eq!(false, world.get(0, 2));
        assert_eq!(true, world.get(1, 2));
        assert_eq!(false, world.get(2, 2));
    }
}
