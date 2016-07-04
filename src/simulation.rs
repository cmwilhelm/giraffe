extern crate rand;

use giraffe;
use giraffe::Giraffe;
use world::World;


const WORLD_SIZE:       u16 = 1000;
const TREE_HEIGHT:      u32 = 1500;
const MUTATION_PERCENT: f32 = 0.001;

pub fn build_initial_world() -> World {
    let tower: Vec<Giraffe> = (0..WORLD_SIZE).map(|_| {
        let legs: Vec<u8> = (0..giraffe::LEG_SEGMENTS).map(|_| {
            rand::random::<u8>()
        }).collect();

        let neck: Vec<u8> = (0..giraffe::NECK_SEGMENTS).map(|_| {
            rand::random::<u8>()
        }).collect();

        Giraffe { legs: legs, neck: neck }
    }).collect();

    World {
        tower:            tower,
        mutation_percent: MUTATION_PERCENT,
        tree_height:      TREE_HEIGHT
    }
}

pub fn evolve_world(world: World) -> World {
    let fitnesses: Vec<f32> = calculate_fitnesses(
        &world,
        &world.tower
    );

    let (cumulative_densities, total_density) =
        generate_cumulative_densities(fitnesses);

    let tower: Vec<Giraffe> = (0..WORLD_SIZE).map(|_| {
        let giraffe1 = select_giraffe(
            &cumulative_densities,
            total_density,
            &world.tower
        );

        let giraffe2 = select_giraffe(
            &cumulative_densities,
            total_density,
            &world.tower
        );

        mate_giraffes(&world, giraffe1, giraffe2)
    }).collect();

    let tree_height = if random_proportion() < 0.0001 {
        (random_proportion() * 1500.0 + 500.0) as u32
    } else {
        world.tree_height
    };

    World {
        tower:            tower,
        mutation_percent: world.mutation_percent,
        tree_height:      tree_height
    }
}

fn generate_cumulative_densities(fitnesses: Vec<f32>) -> (Vec<f64>, f64) {
    let mut total: f64 = 0.0;
    let mut cds        = vec![];

    for fitness in fitnesses.iter() {
        cds.push(total);
        total = total + (*fitness as f64);
    }

    (cds, total)
}

fn calculate_fitnesses(world: &World, tower: &Vec<Giraffe>) -> Vec<f32> {
    let height_deltas: Vec<i32> = tower.iter().map(|giraffe| {
        calculate_tree_delta(&world, &giraffe)
    }).collect();

    let max_delta = match height_deltas.iter().max() {
        Some(m) => *m,
        _       => 0
    };

    height_deltas.iter().map(|delta| {
        ((max_delta - *delta) as f32).cbrt().sqrt()
    }).collect::<Vec<f32>>()
}

fn calculate_tree_delta(world: &World, giraffe: &Giraffe) -> i32 {
    let mut total_height: i32 = 0;

    for l1 in giraffe.legs.iter() {
        total_height = total_height + (*l1 as i32);
    }

    for l2 in giraffe.neck.iter() {
        total_height = total_height + (*l2 as i32);
    }

    (world.tree_height as i32 - total_height as i32).abs()
}

fn select_giraffe<'a>(cumulative_densities: &Vec<f64>, total_density: f64, tower: &'a Vec<Giraffe>) -> &'a Giraffe {
    let mut range_start = 0;
    let mut range_end   = cumulative_densities.len();
    let mut done        = false;

    let search_value = (random_proportion() as f64) * (total_density);

    let mut current: usize = 0;

    while !done {
        current = (range_start + range_end) / 2;
        if search_value >= cumulative_densities[current] {
            let upper_bound;

            if cumulative_densities.len() == current + 1 {
                upper_bound = total_density + 1.0;
            } else {
                upper_bound = cumulative_densities[current + 1]
            }

            if search_value < upper_bound {
                done = true;
            } else {
                range_start = current;
            }
        } else {
            range_end = current;
        }

        if range_end == range_start {
            done = true;
        }
    }

    &tower[current]
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
        if random_proportion() * 100.0 <= world.mutation_percent as f32 {
            rand::random::<u8>()
        } else {
            *i
        }
    }).collect()
}

fn blend_characteristics(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    a.iter().zip(b.iter()).map(|(i, j)| {
        if random_proportion() <= 0.5 {
            *i
        } else {
            *j
        }
    }).collect::<Vec<u8>>()
}

fn random_proportion() -> f32 {
    let result = (rand::random::<u8>() as f32) / ((!0 as u8) as f32);
    result
}
