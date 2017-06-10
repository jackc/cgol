extern crate gol;

use gol::World;
use std::{thread, time};

fn main() {
    let mut world = World::new(16, 16);
    world.populate_rand(0.3);

    loop {
        print_world(&world);
        world.step();
        thread::sleep(time::Duration::from_millis(100));
    }
}

fn print_world(world: &World) {
    print!("{}[2J", 27 as char);
    for y in 0..world.height() {
        for x in 0..world.width() {
            let chr = if world.get(x, y) { "*" } else { " " };
            print!("{}", chr);
        }
        println!();
    }
}


