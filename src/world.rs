extern crate rand;

use giraffe;
use giraffe::Giraffe;


const WORLD_SIZE:       u16 = 1000;
const TREE_HEIGHT:      u32 = 48;
const MUTATION_PERCENT: u8  = 10;


pub struct World {
    giraffes:         Vec<Giraffe>,
    mutation_percent: u8,
    tree_height:      u32
}

pub fn build_world() -> World {
    let giraffes: Vec<Giraffe> = (0..WORLD_SIZE).map(|_| {
        let legs: Vec<u8> = (0..giraffe::LEG_SEGMENTS).map(|_| {
            rand::random::<u8>()
        }).collect();

        let neck: Vec<u8> = (0..giraffe::NECK_SEGMENTS).map(|_| {
            rand::random::<u8>()
        }).collect();

        Giraffe { legs: legs, neck: neck }
    }).collect();

    World {
        giraffes:         giraffes,
        mutation_percent: MUTATION_PERCENT,
        tree_height:      TREE_HEIGHT
    }
}
