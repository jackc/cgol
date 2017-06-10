extern crate gol;
extern crate rustbox;

use gol::World;
use std::time;
use std::error::Error;
use std::default::Default;
use rustbox::{Color, RustBox};
use rustbox::Key;

fn main() {
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };
    rustbox.present();

    let mut world = World::new(rustbox.width() as i16, rustbox.height() as i16);
    world.populate_rand(0.3);

    loop {
        print_world(&rustbox, &world);
        world.step();

        match rustbox.peek_event(time::Duration::from_millis(100), false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Char(_) => { break; }
                    _ => { }
                }
            },
            Err(e) => panic!("{}", e.description()),
            _ => { }
        }
    }
}

fn print_world(rustbox: &rustbox::RustBox, world: &World) {
    rustbox.clear();

    for y in 0..world.height() {
        for x in 0..world.width() {
            let chr = if world.get(x, y) { '*' } else { ' ' };
            unsafe {
                rustbox.change_cell((x as usize) * 2, y as usize, chr as u32, Color::White.as_16color(), Color::Black.as_16color());
            }
        }
    }

    rustbox.present();
}


