extern crate rand;

use blending::{BlendingMode, blend_chromosomes};
use mutation;

use traits::CanMate;
use traits::CanMutate;
use traits::HasColor;
use traits::HasHeight;
use traits::HasSpeed;


const LEG_SEGMENTS:   usize = 4;
const NECK_SEGMENTS:  usize = 8;
const COLOR_SEGMENTS: usize = 4;

pub struct Giraffe {
    color: Vec<u8>,
    legs:  Vec<u8>,
    neck:  Vec<u8>
}

impl Giraffe {
    pub fn random() -> Self {
        let color_chromosome: Vec<u8> = (0..COLOR_SEGMENTS).map(|_| {
            rand::random::<u8>()
        }).collect();

        let legs_chromosome: Vec<u8> = (0..LEG_SEGMENTS).map(|_| {
            rand::random::<u8>()
        }).collect();

        let neck_chromosome: Vec<u8> = (0..NECK_SEGMENTS).map(|_| {
            rand::random::<u8>()
        }).collect();

        Giraffe {
            color: color_chromosome,
            legs:  legs_chromosome,
            neck:  neck_chromosome
        }
    }

    pub fn new_from_phenotypic_values(mut color: u32, mut legs: u32, mut neck: u32) -> Self {
        let mut color_chromosome = vec![];
        let mut legs_chromosome  = vec![];
        let mut neck_chromosome  = vec![];

        let gene_max = !0 as u8;

        for _ in 0..COLOR_SEGMENTS {
            let to_push: u8;

            if color >= gene_max as u32 {
                to_push = gene_max;
                color -= gene_max as u32;
            } else {
                to_push = color as u8;
                color = 0;
            }

            color_chromosome.push(to_push);
        }

        for _ in 0..LEG_SEGMENTS {
            let to_push: u8;

            if legs >= gene_max as u32 {
                to_push = gene_max;
                legs -= gene_max as u32;
            } else {
                to_push = legs as u8;
                legs = 0;
            }

            legs_chromosome.push(to_push);
        }

        for _ in 0..NECK_SEGMENTS {
            let to_push: u8;

            if neck >= gene_max as u32 {
                to_push = gene_max;
                neck -= gene_max as u32;
            } else {
                to_push = neck as u8;
                neck = 0;
            }

            neck_chromosome.push(to_push);
        }

        Giraffe {
            color: color_chromosome,
            legs:  legs_chromosome,
            neck:  neck_chromosome
        }
    }

    pub fn leg_length(&self) -> u32 {
        self.legs.iter().fold(0, |acc, &value| {
            acc + value as u32
        })
    }

    pub fn neck_length(&self) -> u32 {
        self.neck.iter().fold(0, |acc, &value| {
            acc + value as u32
        })
    }
}

impl CanMutate for Giraffe {
    fn mutate(&self, mutation_rate: f32) -> Self {
        let mutated_color_chromosome = mutation::apply_mutations(
            &self.color, mutation_rate
        );

        let mutated_legs_chromosome = mutation::apply_mutations(
            &self.legs, mutation_rate
        );

        let mutated_neck_chromosome = mutation::apply_mutations(
            &self.neck, mutation_rate
        );

        Giraffe {
            color: mutated_color_chromosome,
            legs:  mutated_legs_chromosome,
            neck:  mutated_neck_chromosome
        }
    }
}

impl CanMate for Giraffe {
    fn mate(giraffe1: &Self, giraffe2: &Self, mutation_rate: f32, blending_mode: BlendingMode) -> Self {
        let mutated1 = giraffe1.mutate(mutation_rate);
        let mutated2 = giraffe2.mutate(mutation_rate);

        Giraffe {
            color: blend_chromosomes(&mutated1.color, &mutated2.color, blending_mode),
            legs:  blend_chromosomes(&mutated1.legs, &mutated2.legs, blending_mode),
            neck:  blend_chromosomes(&mutated1.neck, &mutated2.neck, blending_mode)
        }
    }
}

impl HasColor for Giraffe {
    fn color(&self) -> u32 {
        self.color.iter().fold(0, |acc, &x| {
            acc + x as u32
        })
    }
}

impl HasHeight for Giraffe {
    fn height(&self) -> u32 {
        self.leg_length() + self.neck_length()
    }
}

impl HasSpeed for Giraffe {
    fn speed(&self) -> u32 {
        let leg_length  = self.leg_length() as f32;
        let neck_length = self.neck_length() as f32;

        let weight = 4.0 * leg_length + neck_length;
        let speed  = leg_length * 4.0 - 0.0001 * weight * weight;

        if speed <= 0f32 {
            0u32
        } else {
            speed as u32
        }
    }
}
