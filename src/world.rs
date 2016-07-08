use std::cmp::Ordering;

use blending::BlendingMode;
use giraffe::Giraffe;
use giraffe_lib::random_proportion;
use traits::CanMate;
use traits::HasColor;
use traits::HasHeight;
use traits::HasSpeed;


const WORLD_SIZE:    u16          = 1000;
const TREE_HEIGHT:   u32          = 1500;
const MUTATION_RATE: f32          = 0.001;
const LION_SPEED:    u32          = 500;
const COLOR:         u32          = 500;
const BLENDING_MODE: BlendingMode = BlendingMode::OnePointCrossover;

pub struct World {
    pub color:       u32,
    pub generation:  u32,
    pub lion_speed:  u32,
    pub tower:       Vec<Giraffe>,
    pub tree_height: u32,
    mutation_rate:   f32,
    blending_mode:   BlendingMode
}

impl World {
    pub fn new() -> Self {
        let tower: Vec<Giraffe> = (0..WORLD_SIZE).map(|_| {
            Giraffe::random()
        }).collect();

        World::new_from_tower(tower)
    }

    pub fn new_from_tower(tower: Vec<Giraffe>) -> Self {
        World {
            color:         COLOR,
            tower:         tower,
            lion_speed:    LION_SPEED,
            mutation_rate: MUTATION_RATE,
            blending_mode: BLENDING_MODE,
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

            Giraffe::mate(giraffe1, giraffe2, self.mutation_rate, self.blending_mode)
        }).collect();

        let tree_height = if random_proportion() < 0.0001 {
            (random_proportion() * 1500.0 + 500.0) as u32
        } else {
            self.tree_height
        };

        World {
            color:         self.color,
            tower:         tower,
            lion_speed:    self.lion_speed,
            mutation_rate: self.mutation_rate,
            blending_mode: self.blending_mode,
            tree_height:   tree_height,
            generation:    self.generation + 1
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
        (giraffe.color()  as i32, world.color       as i32, 1.0 as f32),
        (giraffe.height() as i32, world.tree_height as i32, 1.5 as f32),
        (giraffe.speed()  as i32, world.lion_speed  as i32, 1.0 as f32)
    ]
        .into_iter()
        .map(|(phenotype, environment, weight)| {
            let delta = (phenotype - environment).abs();
            let score = match delta {
                d if d <= environment => (environment - d) as f32 / environment as f32,
                _ => 0.0
            };

            score * weight as f32
        })
        .fold(0.0, |acc, proportion| {
            (acc + proportion) as f32
        })
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
