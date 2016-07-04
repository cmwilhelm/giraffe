use std::cmp::Ordering;

use giraffe::Giraffe;
use giraffe_lib::random_proportion;
use traits::CanMate;
use traits::HasHeight;


const WORLD_SIZE:    u16 = 1000;
const TREE_HEIGHT:   u32 = 1500;
const MUTATION_RATE: f32 = 0.001;

pub struct World {
    pub tower:       Vec<Giraffe>,
    pub tree_height: u32,
    pub generation:  u32,
    mutation_rate:   f32
}

impl World {
    pub fn new() -> Self {
        let tower: Vec<Giraffe> = (0..WORLD_SIZE).map(|_| {
            Giraffe::random()
        }).collect();

        World {
            tower:         tower,
            mutation_rate: MUTATION_RATE,
            tree_height:   TREE_HEIGHT,
            generation:    0
        }
    }

    pub fn evolve(&self) -> Self {
        let fitnesses: Vec<f32> = calculate_fitnesses(
            &self,
            &self.tower
        );

        let cumulative_densities = generate_cumulative_densities(fitnesses);

        let tower: Vec<Giraffe> = (0..WORLD_SIZE).map(|_| {
            let giraffe1 = select_giraffe(
                &cumulative_densities,
                &self.tower
            );

            let giraffe2 = select_giraffe(
                &cumulative_densities,
                &self.tower
            );

            Giraffe::mate(giraffe1, giraffe2, self.mutation_rate)
        }).collect();

        let tree_height = if random_proportion() < 0.0001 {
            (random_proportion() * 1500.0 + 500.0) as u32
        } else {
            self.tree_height
        };

        World {
            tower:         tower,
            mutation_rate: self.mutation_rate,
            tree_height:   tree_height,
            generation:    self.generation + 1
        }
    }
}


fn generate_cumulative_densities(fitnesses: Vec<f32>) -> Vec<(f64, f64)> {
    let mut total: f64 = 0.0;
    let mut cds        = vec![];


    for i in 0..fitnesses.len() {
        let first = total;
        total = total + (fitnesses[i] as f64);

        let second = if i == fitnesses.len() - 1 {
            total + 1.0
        } else {
            total
        };

        cds.push( (first, second) );
    }


    cds
}

fn select_giraffe<'a>(cumulative_densities: &Vec<(f64, f64)>, tower: &'a Vec<Giraffe>) -> &'a Giraffe {
    let search_value  = (random_proportion() as f64) * (cumulative_densities.last().unwrap().1 - 1.0);
    let search_result = cumulative_densities.binary_search_by(|&(min, max)| {
        if min > search_value {
            Ordering::Greater
        } else {
            if search_value > max {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        }
    });

    let matching_index = match search_result {
        Ok(i) => i,
        _     => 0
    };

    &tower[matching_index]
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
    (world.tree_height as i32 - giraffe.height() as i32).abs()
}
