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

pub fn evolve_world(world: World) -> World {
    let fitnesses: Vec<f32> = world.giraffes.iter().map(|giraffe| {
        calculate_fitness(&world, &giraffe)
    }).collect();

    let cumulative_densities = generate_cumulative_densities(fitnesses);

    let giraffes: Vec<Giraffe> = (0..WORLD_SIZE).map(|_| {
        let giraffe1 = select_giraffe(&cumulative_densities, &world.giraffes);
        let giraffe2 = select_giraffe(&cumulative_densities, &world.giraffes);

        mate_giraffes(giraffe1, giraffe2)
    }).collect();

    World {
        giraffes:         giraffes,
        mutation_percent: world.mutation_percent,
        tree_height:      world.tree_height
    }
}

pub fn generate_cumulative_densities(fitnesses: Vec<f32>) -> Vec<f32>{
    let mut total = 0.0;
    let mut cds   = vec![];

    for fitness in fitnesses.iter() {
        total = total + fitness;
        cds.push(total);
    }

    cds
}

pub fn calculate_fitness(world: &World, giraffe: &Giraffe) -> f32 {
    1.0
}

pub fn select_giraffe<'a>(cumulative_densities: &Vec<f32>, giraffes: &'a Vec<Giraffe>) -> &'a Giraffe {
    &giraffes[0]
}

pub fn mate_giraffes(giraffe1: &Giraffe, giraffe2: &Giraffe) -> Giraffe {
    Giraffe {
        legs: giraffe1.legs.clone(),
        neck: giraffe1.neck.clone()
    }
}
