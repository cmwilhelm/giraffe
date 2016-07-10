use std::cmp::Ordering;

use blending::BlendingMode;
use defaults;
use giraffe::Giraffe;
use giraffe_lib::random_proportion;
use mutation::{MutationDecay, calculate_mutation_rate};
use options;
use traits::CanMate;
use traits::HasColor;
use traits::HasHeight;
use traits::HasSpeed;


pub struct World {
    pub generation: u32,
    pub options:    options::Options,
    pub tower:      Vec<Giraffe>
}

impl World {
    pub fn new(options: options::Options) -> Self {
        let tower: Vec<Giraffe> = (0..defaults::WORLD_SIZE).map(|_| {
            Giraffe::random()
        }).collect();

        World::new_from_tower(tower, options)
    }

    pub fn new_from_tower(tower: Vec<Giraffe>, options: options::Options) -> Self {
        World {
            generation: 0,
            options:    options,
            tower:      tower
        }
    }

    pub fn evolve(&self) -> Self {
        let mutation_rate = calculate_mutation_rate(
            self.options.mutation_decay,
            self.options.mutation_rate,
            self.generation + 1,
            self.options.simulation_length
        );

        let fitnesses: Vec<f32> = calculate_fitnesses(
            &self,
            &self.tower
        );

        let cumulative_densities = generate_cumulative_densities(fitnesses);

        let tower: Vec<Giraffe> = (0..defaults::WORLD_SIZE).map(|_| {
            let giraffe1 = select_giraffe(
                &cumulative_densities,
                &self.tower
            );

            let giraffe2 = select_giraffe(
                &cumulative_densities,
                &self.tower
            );

            Giraffe::mate(giraffe1, giraffe2, mutation_rate, self.options.blending_mode)
        }).collect();

        let tree_height = if random_proportion() < 0.0001 {
            (random_proportion() * 1500.0 + 500.0) as u32
        } else {
            self.options.tree_height
        };

        World {
            generation: self.generation + 1,
            options:    options::Options { tree_height: tree_height, ..self.options },
            tower:      tower
        }
    }
}

pub fn calculate_fitnesses(world: &World, tower: &Vec<Giraffe>) -> Vec<f32> {
    tower.iter().map(|giraffe| {
        calculate_fitness(&world, &giraffe)
    }).collect::<Vec<f32>>()
}

fn calculate_fitness(world: &World, giraffe: &Giraffe) -> f32 {
    vec![
        (giraffe.color()  as i32, world.options.color       as i32, 1.0),
        (giraffe.height() as i32, world.options.tree_height as i32, 1.5),
        (giraffe.speed()  as i32, world.options.lion_speed  as i32, 1.0)
    ]
        .into_iter()
        .map(|(phenotype, environment, weight)| {
            calculate_partial_fitness(phenotype, environment, weight)
        })
        .fold(0.0, |acc, proportion| acc + proportion)
}

fn calculate_partial_fitness(phenotype: i32, environment: i32, weight: f32) -> f32 {
    let delta = (phenotype - environment).abs();
    let score = match delta {
        d if d <= environment => (environment - d) as f32 / environment as f32,
        _ => 0.0
    };

    score * weight as f32
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
