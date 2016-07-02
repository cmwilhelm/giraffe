extern crate rand;

use giraffe;
use giraffe::Giraffe;
use world::World;


const WORLD_SIZE:       u16 = 1000;
const TREE_HEIGHT:      u32 = 48;
const MUTATION_PERCENT: u8  = 10;

pub fn build_initial_world() -> World {
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
