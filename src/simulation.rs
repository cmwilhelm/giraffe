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

        mate_giraffes(&world, giraffe1, giraffe2)
    }).collect();

    World {
        giraffes:         giraffes,
        mutation_percent: world.mutation_percent,
        tree_height:      world.tree_height
    }
}

fn generate_cumulative_densities(fitnesses: Vec<f32>) -> Vec<f32>{
    let mut total = 0.0;
    let mut cds   = vec![];

    for fitness in fitnesses.iter() {
        total = total + fitness;
        cds.push(total);
    }

    cds
}

fn calculate_fitness(world: &World, giraffe: &Giraffe) -> f32 {
    let mut total_height: u32 = 0;

    for l1 in giraffe.legs.iter() {
        total_height = total_height + (*l1 as u32);
    }

    for l2 in giraffe.legs.iter() {
        total_height = total_height + (*l2 as u32);
    }

    (world.tree_height as f32 - total_height as f32).abs()
}

fn select_giraffe<'a>(cumulative_densities: &Vec<f32>, giraffes: &'a Vec<Giraffe>) -> &'a Giraffe {
    &giraffes[0]
}

fn mate_giraffes(world: &World, giraffe1: &Giraffe, giraffe2: &Giraffe) -> Giraffe {
    let legs1 = apply_mutations(&world, &giraffe1.legs);
    let legs2 = apply_mutations(&world, &giraffe2.legs);
    let neck1 = apply_mutations(&world, &giraffe1.neck);
    let neck2 = apply_mutations(&world, &giraffe2.neck);

    Giraffe {
        legs: blend_characteristics(&legs1, &legs2),
        neck: blend_characteristics(&neck1, &neck2)
    }
}

fn apply_mutations(world: &World, characteristics: &Vec<u8>) -> Vec<u8> {
    characteristics.iter().map(|i| {
        if (rand::random::<u8>() / (!0 as u8)) * 100 > world.mutation_percent {
            *i
        } else {
            rand::random::<u8>()
        }
    }).collect()
}

fn blend_characteristics(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    a.iter().zip(b.iter()).map(|(i, j)| {
        let total: u16 = (*i as u16) + (*j as u16);

        (total / 2) as u8
    }).collect()
}
